mod api;
mod player_events;
mod player_options;
mod player_state;

use alloc::{borrow::ToOwned, boxed::Box, rc::Rc, string::String, vec, vec::Vec};
use core::{cell::RefCell, ops::Deref};

use crate::{controllable_promise, init_yt_api, PromiseConstructorFunction};

pub use self::player_events::PlayerEvents;
pub use self::player_options::{PlayerOptions, PlayerVars};
pub use self::player_state::PlayerState;

use self::api::PlayerInstance;

use hashbrown::HashMap;
use js_sys::{Array, Function, Object, Promise, Reflect};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{console, window};

#[derive(Debug)]
struct EventHandler {
    namespace: Option<String>,
    handler: Rc<Function>,
}

type EventHandlerHashmap = Rc<RefCell<HashMap<String, Vec<EventHandler>>>>;

#[wasm_bindgen(js_name = YoutubePlayer)]
#[derive(Debug)]
pub struct YtPlayer {
    is_ready: Rc<RefCell<bool>>,
    player_loaded: Rc<Promise>,
    player_instance: Option<PlayerInstance>,
    event_handlers: EventHandlerHashmap,
}

#[wasm_bindgen(js_class = YoutubePlayer)]
impl YtPlayer {
    #[wasm_bindgen(constructor)]
    pub fn new(player_id: &str, options: Object) -> Self {
        // create new or use existing options object
        let mut options_object = Object::new();

        let player_options: Object = options;
        let checkable_options = JsValue::from(player_options.clone());

        if !checkable_options.is_undefined() && !checkable_options.is_null() {
            options_object = player_options;
        }

        // prepare flags and function to signal a ready player
        let is_ready_handle = Rc::new(RefCell::new(false));
        let (player_ready, ready_resolver, ready_rejecter) = controllable_promise();

        let handlers: EventHandlerHashmap = Rc::new(RefCell::new(HashMap::new()));

        // add own wrapper ready event handler to list (loading signal for promise)
        let new_handler = Self::create_ready_event_handler(is_ready_handle.clone(), ready_resolver);

        Self::add_event_handler_fn(
            None,
            handlers.clone(),
            ("ready", None),
            new_handler.into_js_value().unchecked_into::<Function>(),
        );

        // read given events from options
        let previous_events = Reflect::get(&options_object, &"events".into()).ok();

        if let Some(previous_events) = previous_events {
            if !previous_events.is_undefined() && !previous_events.is_null() {
                // delete old event property, don't forward to original API
                let _success = Reflect::delete_property(&options_object, &"events".into()).unwrap();

                // extract given events and add them to wrapper hashmap
                let event_options = previous_events.dyn_into::<Object>();

                if let Ok(event_options) = event_options {
                    let property_names = Object::get_own_property_names(&event_options);

                    for property_name in property_names.iter() {
                        let event_name: String = from_value(property_name).unwrap();

                        let descriptor = Object::get_own_property_descriptor(
                            &event_options,
                            &event_name.clone().into(),
                        );

                        let descriptor_value = Reflect::get(&descriptor, &"value".into()).unwrap();

                        if let Ok(handler_fn) = descriptor_value.dyn_into::<Function>() {
                            let namespaced_event = PlayerEvents::get_namespaced_event(&event_name);

                            match namespaced_event {
                                Ok(namespaced_event) => {
                                    Self::add_event_handler_fn(
                                        None,
                                        handlers.clone(),
                                        namespaced_event,
                                        handler_fn,
                                    );
                                }
                                Err(error) => console::error_1(&error.into()),
                            }
                        }
                    }
                }
            }
        }

        // add all wrapper events to options
        // events, like ready event, are provided before initialization
        let events_object = Object::new();

        for handler in handlers.deref().borrow().iter() {
            let handler_name = PlayerEvents::get_handler_name(handler.0);
            let handler_wrapper =
                Self::create_event_handler_wrapper(handlers.clone(), handler.0).into_js_value();

            match handler_name {
                Ok(handler_name) => {
                    if let Err(error) =
                        Reflect::set(&events_object, &handler_name.into(), &handler_wrapper)
                    {
                        console::error_1(&error);
                    }
                }
                Err(error) => console::error_1(&error.into()),
            }
        }

        let _success = Reflect::set(&options_object, &"events".into(), &events_object);

        // create a youtube player instance
        let yt_global = Reflect::get(&window().unwrap(), &"YT".into()).unwrap();
        let player_constructor = Reflect::get(&yt_global, &"Player".into()).unwrap();

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
            event_handlers: handlers,
        }
    }

    pub fn create(player_id: &str, options: Object) -> Promise {
        let player_id = player_id.to_owned();

        future_to_promise(async move {
            let _is_api_ready = JsFuture::from(init_yt_api()).await?;

            let instance = Self::new(&player_id, options);

            let player_init_complete = instance.player_loaded.deref().clone();
            let _is_player_ready = JsFuture::from(player_init_complete).await?;

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

    fn create_ready_event_handler(
        is_ready: Rc<RefCell<bool>>,
        ready_resolver: PromiseConstructorFunction,
    ) -> Closure<dyn FnMut(JsValue)> {
        Closure::wrap(Box::new(move |_event: JsValue| {
            *is_ready.deref().borrow_mut() = true;

            console::info_1(&"Player Instance Ready".into());

            // signal player loading complete
            ready_resolver
                .borrow()
                .as_ref()
                .unwrap()
                .apply(&JsValue::null(), &Array::new())
                .unwrap();
        }) as Box<dyn FnMut(JsValue)>)
    }

    fn create_event_handler_wrapper(
        handler_hashmap: EventHandlerHashmap,
        event_name: &str,
    ) -> Closure<dyn FnMut(JsValue)> {
        let event_name = event_name.to_owned();

        let target_property = "target";

        Closure::wrap(Box::new(move |event: JsValue| {
            // remove target property (unwrapped Youtube API instance) from event
            if Reflect::get(&event, &target_property.into()).is_ok() {
                let _success = Reflect::delete_property(
                    event.dyn_ref::<Object>().unwrap(),
                    &target_property.into(),
                )
                .unwrap();
            }

            // only use event as parameter, if it still contains other data
            let params = if Reflect::own_keys(&event).unwrap().length() > 0 {
                Array::from_iter([event])
            } else {
                Array::new()
            };

            for handler in handler_hashmap.borrow().get(&event_name).unwrap().iter() {
                handler.handler.apply(&JsValue::null(), &params).unwrap();
            }
        }) as Box<dyn FnMut(JsValue)>)
    }

    fn add_event_handler_fn(
        instance: Option<&PlayerInstance>,
        handler_hashmap: EventHandlerHashmap,
        namespaced_event: (&str, Option<&str>),
        handler_fn: Function,
    ) {
        let (event_name, namespace) = namespaced_event;

        let handler_name = PlayerEvents::get_handler_name(event_name);

        match handler_name {
            Ok(handler_name) => {
                let mut hashmap = handler_hashmap.deref().borrow_mut();

                if !hashmap.contains_key(event_name) {
                    hashmap.insert(event_name.to_owned(), vec![]);

                    if let Some(instance) = instance {
                        // add event handler wrapper to original Youtube API, if it has a brandnew key
                        // doesn't use hashmap.entry(…).or_insert(…) with check for empty vector,
                        // because vector could be empty after removing events too
                        instance.add_event_listener(
                            handler_name.into(),
                            Self::create_event_handler_wrapper(handler_hashmap.clone(), event_name)
                                .into_js_value(),
                        );
                    }
                }

                let handler_vec = hashmap.get_mut(event_name).unwrap();

                // add event handler to handler list
                handler_vec.push(EventHandler {
                    namespace: namespace.map(|ns| ns.to_owned()),
                    handler: Rc::new(handler_fn),
                });
            }
            Err(error) => console::error_1(&error.into()),
        }
    }

    pub fn on(&self, event_name: &str, handler_fn: JsValue) {
        let namespaced_event = PlayerEvents::get_namespaced_event(event_name);

        if let Err(error) = namespaced_event {
            console::error_1(&error.into());
            return;
        }

        if let Ok(handler_fn) = handler_fn.dyn_into::<Function>() {
            self.run_player(move |instance| {
                Self::add_event_handler_fn(
                    Some(instance),
                    self.event_handlers.clone(),
                    namespaced_event.unwrap(),
                    handler_fn,
                );
            });
        }
    }

    pub fn off(&self, event_name: &str) {
        let namespaced_event = PlayerEvents::get_namespaced_event(event_name);

        if let Err(error) = namespaced_event {
            console::error_1(&error.into());
            return;
        }

        let (event_name, namespace) = namespaced_event.unwrap();

        let mut hashmap = self.event_handlers.deref().borrow_mut();

        if hashmap.contains_key(event_name) {
            let handler_vec = hashmap.get_mut(event_name).unwrap();

            match namespace {
                Some(namespace) => {
                    // remove all elements containing the namespace
                    handler_vec.retain(|handler| {
                        if let Some(handler_namespace) = &handler.namespace {
                            return handler_namespace != namespace;
                        }

                        true
                    });
                }
                None => handler_vec.clear(),
            }
        }
    }

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
