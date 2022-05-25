#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod wrapper;

use alloc::boxed::Box;
use alloc::sync::Arc;
use core::cell::RefCell;

use js_sys::Promise;
use js_sys::{Array, Function, Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window};

pub use wrapper::{PlayerEvents, PlayerOptions, PlayerState, PlayerVars, YtPlayer};

// #[cfg(feature = "wee_alloc")]
// // Use `wee_alloc` as the global allocator.
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// called when the wasm module is instantiated
// #[wasm_bindgen(start)]
#[wasm_bindgen(js_name = initYtApi)]
pub fn init_yt_api() -> Promise {
    #[cfg(feature = "std")]
    // enable feature "std" to show rust stack trace instead of cryptic "RuntimeError: unreachable executed"
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = window().unwrap();

    // create promise to signal when library has successfully initialized youtube player api
    let (api_ready, init_resolver, _) = controllable_promise();

    // check and save if there's already a ready handler function
    let previous_ready_function =
        Reflect::get(&window, &to_value("onYouTubeIframeAPIReady").unwrap()).unwrap();

    // create ready handler function specific for library
    let new_handler = Closure::wrap(Box::new(move || {
        // execute custom code for library
        console::log_1(&"Youtube Player API ready".into());

        // signal api loading complete
        init_resolver
            .borrow()
            .as_ref()
            .unwrap()
            .apply(&JsValue::null(), &Array::new())
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
                &to_value("onYouTubeIframeAPIReady").unwrap(),
                &ready_fn_to_restore.as_ref(),
            )
            .unwrap();
        }
    }) as Box<dyn FnMut()>);

    // put library ready handler in place,
    // it will restore original ready handler if needed
    let _success = Reflect::set(
        &window,
        &to_value("onYouTubeIframeAPIReady").unwrap(),
        &new_handler.into_js_value(),
    )
    .unwrap();

    // load the IFrame Player API code asynchronously
    let document = window.document().unwrap();

    let api_script = document.create_element("script").unwrap();
    api_script
        .set_attribute("src", "https://www.youtube.com/player_api")
        .unwrap();

    let first_script_tag = document.get_elements_by_tag_name("script").item(0).unwrap();
    first_script_tag
        .parent_node()
        .unwrap()
        .insert_before(&api_script, Some(&first_script_tag))
        .unwrap();

    api_ready
}

fn controllable_promise() -> (
    Promise,
    Arc<RefCell<Option<Function>>>,
    Arc<RefCell<Option<Function>>>,
) {
    let resolve_function: Arc<RefCell<Option<Function>>> = Arc::new(RefCell::new(None));
    let reject_function: Arc<RefCell<Option<Function>>> = Arc::new(RefCell::new(None));

    let promise_resolve = resolve_function.clone();
    let promise_reject = reject_function.clone();

    let promise = Promise::new(&mut move |resolve, reject| {
        promise_resolve.replace(Some(resolve));
        promise_reject.replace(Some(reject));
    });

    (promise, resolve_function, reject_function)
}
