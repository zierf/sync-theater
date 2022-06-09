mod api;
mod player_events;
mod player_options;
mod player_state;

use alloc::{boxed::Box, rc::Rc};
use core::{cell::RefCell, ops::Deref};

use crate::controllable_promise;

pub use self::player_events::PlayerEvents;
pub use self::player_options::{PlayerOptions, PlayerVars};
pub use self::player_state::PlayerState;

use self::api::PlayerInstance;

use js_sys::{Array, Function, Object, Promise, Reflect};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{console, window};

#[wasm_bindgen(js_name = YoutubePlayer)]
#[derive(Debug)]
pub struct YtPlayer {
    is_ready: Rc<RefCell<bool>>,
    player_loaded: Rc<Promise>,
    player_instance: Option<PlayerInstance>,
}

#[wasm_bindgen(js_class = YoutubePlayer)]
impl YtPlayer {
    #[wasm_bindgen(constructor)]
    pub fn new(player_id: &str, options: Object) -> Self {
        let window = window().unwrap();

        let yt_global = Reflect::get(&window, &"YT".into()).unwrap();
        let player_constructor = Reflect::get(&yt_global, &"Player".into()).unwrap();

        let is_ready_handle = Rc::new(RefCell::new(false));
        let is_ready_closure = is_ready_handle.clone();

        let (player_ready, ready_resolver, ready_rejecter) = controllable_promise();

        let player_options: Object = options;
        let checkable_options = JsValue::from(player_options.clone());

        let mut options_object = Object::new();

        if !checkable_options.is_undefined() && !checkable_options.is_null() {
            options_object = player_options;
        }

        let mut event_options = Object::new();

        let previous_events = Reflect::get(&options_object, &"events".into()).ok();
        let checkable_events = JsValue::from(previous_events.clone());

        let previous_ready_handler = if let Some(previous_events) = previous_events {
            if !checkable_events.is_undefined() && !checkable_events.is_null() {
                event_options = previous_events.unchecked_into::<Object>();
            }

            Reflect::get(&event_options, &"onReady".into()).ok()
        } else {
            None
        };

        let new_handler = add_ready_event_handler(move |player_instance: &PlayerInstance| {
            *is_ready_closure.deref().borrow_mut() = true;

            console::log_1(&"Player Ready".into());

            // signal player loading complete
            ready_resolver
                .borrow()
                .as_ref()
                .unwrap()
                .apply(&JsValue::null(), &Array::new())
                .unwrap();

            if let Some(ready_handler_to_call) = previous_ready_handler
                .as_ref()
                .unwrap()
                .dyn_ref::<Function>()
            {
                ready_handler_to_call
                    .apply(&JsValue::null(), &Array::from_iter([player_instance]))
                    .unwrap();
            }
        });

        let _success = Reflect::set(
            &event_options,
            &"onReady".into(),
            &new_handler.into_js_value(),
        );

        let _success = Reflect::set(&options_object, &"events".into(), &event_options);

        // create a youtube player instance
        let player_instance = Reflect::construct(
            player_constructor.dyn_ref::<Function>().unwrap(),
            &Array::from_iter([JsValue::from(player_id), options_object.into()]),
        )
        .unwrap();

        // player member "i" is not null on successful binding
        let player_successful = Reflect::get(&player_instance, &"i".into()).unwrap();

        let player_instance = if !player_successful.is_undefined() && !player_successful.is_null() {
            Some(player_instance.unchecked_into::<PlayerInstance>())
        } else {
            // signal player loading failed
            ready_rejecter
                .borrow()
                .as_ref()
                .unwrap()
                .apply(&JsValue::null(), &Array::new())
                .unwrap();

            None
        };

        Self {
            is_ready: is_ready_handle,
            player_loaded: Rc::new(player_ready),
            player_instance,
        }
    }

    pub fn create(player_id: &str, options: Object) -> Promise {
        let instance = Self::new(player_id, options);

        let player_init_complete = instance.player_loaded.deref().clone();

        let is_ready = JsFuture::from(player_init_complete);

        future_to_promise(async move {
            let _is_player_ready = is_ready.await?;

            Ok(instance.into())
        })
    }

    fn get_player_instance(&self) -> Option<&PlayerInstance> {
        if *self.is_ready.borrow() {
            return self.player_instance.as_ref();
        }

        console::warn_1(&"Player isn't ready yet!".into());

        None
    }

    fn run_player<F>(&self, cb: F)
    where
        F: FnOnce(&PlayerInstance),
    {
        let player_instance_option = self.get_player_instance();
        player_instance_option.map(cb);
    }

    pub fn on(&self, event_name: &str, handler_fn: JsValue) {
        let handler_name = PlayerEvents::get_handler_name(event_name);

        match handler_name {
            Ok(handler_name) =>
                self.run_player(|instance| instance.add_event_listener(handler_name.into(), handler_fn)),
            Err(error) => {
                console::error_1(&error.into())
            }
        }
    }

    // FIXME find a way around broken removeEventListener(...)
    // see https://issuetracker.google.com/issues/35175764
    /*pub fn off(&self, event_name: &str, handler_fn: JsValue) {
        let handler_name = PlayerEvents::get_handler_name(event_name);
        self.run_player(|instance| instance.remove_event_listener(handler_name.into(), handler_fn));
    }*/

    #[wasm_bindgen(js_name = playVideo)]
    pub fn play_video(&self) {
        self.run_player(|instance| instance.play_video());
    }

    #[wasm_bindgen(js_name = pauseVideo)]
    pub fn pause_video(&self) {
        self.run_player(|instance| instance.pause_video());
    }

    #[wasm_bindgen(js_name = stopVideo)]
    pub fn stop_video(&self) {
        self.run_player(|instance| instance.stop_video());
    }

    #[wasm_bindgen(js_name = changeVideo)]
    pub fn change_video(&self, video_id: &str) {
        self.run_player(|instance| instance.cue_video_by_id(video_id.into()));
    }

    #[wasm_bindgen(js_name = getPlayerState)]
    pub fn get_player_state(&self) -> i32 {
        let player_instance_option = self.get_player_instance();

        if let Some(instance) = player_instance_option {
            let state = instance.get_player_state();
            return from_value(state).unwrap();
        }

        PlayerState::UNSTARTED
    }
}

// #[derive(Debug)]
// pub struct YtPlayerOptionsEvents {
//     on_ready: Closure<dyn FnMut(JsValue)>,
//     on_state_change: Closure<dyn FnMut(JsValue)>,
//     on_quality_change: Closure<dyn FnMut(JsValue)>,
//     on_playback_rate_change: Closure<dyn FnMut(JsValue)>,
//     on_error: Closure<dyn FnMut(JsValue)>,
// }

fn add_ready_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
where
    F: 'static,
    F: Fn(&PlayerInstance),
{
    Closure::wrap(Box::new(move |event: JsValue| {
        let event_target = Reflect::get(&event, &"target".into());

        // FIXME find a way to ensure it's a proper player instance
        // let yt_player = event_target.and_then(|event_target| event_target.dyn_into::<PlayerInstance>()).ok();
        // yt_player.map(|player_instance| cb(player_instance));

        if let Ok(event_target) = event_target {
            cb(event_target.unchecked_ref::<PlayerInstance>());
        }
    }) as Box<dyn FnMut(JsValue)>)
}
