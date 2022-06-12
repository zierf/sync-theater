#![warn(missing_debug_implementations, rust_2018_idioms)] // TODO missing_docs
#![cfg_attr(not(feature = "std"), no_std)]

// #![feature(test)]
// extern crate test;

extern crate alloc;

mod wrapper;

use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use js_sys::{try_iter, Array, Function, Object, Promise, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, window, Element};

pub use wrapper::{PlayerEvents, PlayerOptions, PlayerState, PlayerVars, YtPlayer};

#[cfg(feature = "wee_alloc")]
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = initYtApi)]
pub fn init_yt_api() -> Promise {
    #[cfg(feature = "std")]
    // enable feature "std" to show rust stack trace instead of cryptic "RuntimeError: unreachable executed"
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = window().unwrap();
    let document = window.document().unwrap();

    // create promise to signal when library has successfully initialized youtube player api
    let (api_ready, init_resolver, _) = controllable_promise();

    // check if api script was already added
    let api_script_url = "https://www.youtube.com/player_api";
    let mut script_exists = false;

    let script_tags = document.get_elements_by_tag_name("script");

    if let Ok(Some(mut script_iterator)) = try_iter(&script_tags) {
        while let Some(Ok(script_tag)) = script_iterator.next() {
            let element = script_tag.dyn_into::<Element>();

            if let Ok(element) = element {
                let source = element.get_attribute("src");

                if let Some(src) = source {
                    if src.as_str().starts_with(api_script_url) {
                        script_exists = true;
                    }
                }
            }
        }
    }

    // skip loading API if already loaded
    if script_exists {
        let yt_global = get_yt_global();

        // api ready handler already ran before
        if let Ok(yt_global) = yt_global {
            // signal api loading complete
            init_resolver
                .borrow()
                .as_ref()
                .unwrap()
                .apply(&JsValue::null(), &Array::from_iter([yt_global]))
                .unwrap();

            return api_ready;
        }
    }

    // check and save if there's already a ready handler function
    let previous_ready_function = Reflect::get(&window, &"onYouTubeIframeAPIReady".into()).unwrap();

    // create ready handler function specific for wrapper library
    // can also run again if script already exists but YT global object hasn't fully laoded yet
    let new_handler = Closure::wrap(Box::new(move || {
        // execute custom code for library
        console::info_1(&"Youtube Player API ready".into());

        let yt_global = get_yt_global().unwrap();

        // signal api loading complete
        init_resolver
            .borrow()
            .as_ref()
            .unwrap()
            .apply(&JsValue::null(), &Array::from_iter([yt_global]))
            .unwrap();

        // call and restore previous ready handler function
        // after everything is done
        if let Some(ready_fn_to_restore) = previous_ready_function.dyn_ref::<Function>() {
            // call original ready handler function to prevent other scripts from breaking
            ready_fn_to_restore
                .apply(&JsValue::null(), &Array::new())
                .unwrap();

            let _success = Reflect::set(
                &web_sys::window().unwrap(),
                &"onYouTubeIframeAPIReady".into(),
                ready_fn_to_restore.as_ref(),
            )
            .unwrap();
        }
    }) as Box<dyn FnMut()>);

    // put library ready handler in place, will restore the original ready handler if needed
    // multiple nested handlers should restore each other
    let _success = Reflect::set(
        &window,
        &"onYouTubeIframeAPIReady".into(),
        &new_handler.into_js_value(),
    )
    .unwrap();

    // only add api script once
    if !script_exists {
        // load the IFrame Player API code asynchronously
        let document = window.document().unwrap();

        let api_script = document.create_element("script").unwrap();
        api_script.set_attribute("src", api_script_url).unwrap();

        let first_script_tag = document.get_elements_by_tag_name("script").item(0).unwrap();
        first_script_tag
            .parent_node()
            .unwrap()
            .insert_before(&api_script, Some(&first_script_tag))
            .unwrap();
    }

    api_ready
}

type PromiseConstructorFunction = Rc<RefCell<Option<Function>>>;

fn controllable_promise() -> (
    Promise,
    PromiseConstructorFunction,
    PromiseConstructorFunction,
) {
    let resolve_function: PromiseConstructorFunction = Rc::new(RefCell::new(None));
    let reject_function: PromiseConstructorFunction = Rc::new(RefCell::new(None));

    let promise_resolve = resolve_function.clone();
    let promise_reject = reject_function.clone();

    let promise = Promise::new(&mut move |resolve, reject| {
        promise_resolve.replace(Some(resolve));
        promise_reject.replace(Some(reject));
    });

    (promise, resolve_function, reject_function)
}

fn get_yt_global() -> Result<Object, JsValue> {
    let window = window().unwrap();

    let yt_global = Reflect::get(&window, &"YT".into())?;

    if let Ok(yt_global_object) = yt_global.dyn_into::<Object>() {
        let player_constructor = Reflect::get(&yt_global_object, &"Player".into())?;

        if player_constructor.dyn_into::<Function>().is_ok() {
            return Ok(yt_global_object);
        }
    }

    Err(JsValue::undefined())
}
