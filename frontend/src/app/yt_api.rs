use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


#[wasm_bindgen]
extern {
    // #[wasm_bindgen(js_name = setInterval)]
    // pub fn set_interval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;

    // #[wasm_bindgen(js_name = clearInterval)]
    // pub fn clear_interval(id: i32);
}

/*
 * Control youtube player with NPM-Package converted to js module.
 * https://dev.jspm.io/youtube-player@5.5.2
 * http://cdn.skypack.dev/youtube-player@^5.5.2
 * https://cdn.skypack.dev/pin/youtube-player@v5.5.2-Z5kTgZEBuEkovu58iG2z/mode=imports,min/optimized/youtube-player.js
 */
#[wasm_bindgen(module = "https://cdn.skypack.dev/pin/youtube-player@v5.5.2-Z5kTgZEBuEkovu58iG2z/mode=imports,min/optimized/youtube-player.js")]
extern {
    pub type PlayerInstance;

    #[wasm_bindgen(catch, js_name = default)]
    fn YouTubePlayer(player_id: String) -> Result<PlayerInstance, JsValue>;

    #[wasm_bindgen(method, js_class = PlayerInstance, js_name = playVideo)]
    pub fn play_video(this: &PlayerInstance);

    #[wasm_bindgen(method, js_class = PlayerInstance, js_name = pauseVideo)]
    pub fn pause_video(this: &PlayerInstance);

    #[wasm_bindgen(method, js_class = PlayerInstance, js_name = stopVideo)]
    pub fn stop_video(this: &PlayerInstance);

    #[wasm_bindgen(method, js_class = PlayerInstance, js_name = cueVideoById)]
    pub fn change_video(this: &PlayerInstance, video_id: String);

    #[wasm_bindgen(method, catch, js_class = PlayerInstance, js_name = getPlayerState)]
    pub async fn get_player_state(this: &PlayerInstance) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_class = PlayerInstance, js_name = on)]
    pub fn on(this: &PlayerInstance, event: String, closure: &Closure<dyn FnMut(JsValue)>);
}

pub fn bind_player_to_instance(element_id: String) -> Result<PlayerInstance, JsValue> {
    #[allow(unused_unsafe)]
    unsafe {
        return YouTubePlayer(element_id);
    }
}

pub fn add_ready_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
    where F: 'static,
    F: Fn(PlayerInstance) -> ()
{
    Closure::wrap(Box::new(move | event: JsValue | {
        #[allow(unused_unsafe)]
        let event_target = unsafe {
            js_sys::Reflect::get(&event, &wasm_bindgen::JsValue::from_str("target"))
        };

        // FIXME find a way to ensure it's a proper player instance
        // let yt_player = event_target.and_then(|event_target| event_target.dyn_into::<PlayerInstance>()).ok();
        // yt_player.map(|player_instance| cb(player_instance));

        if let Ok(event_target) = event_target {
            cb(event_target.unchecked_into::<PlayerInstance>());
        }
    }) as Box<dyn FnMut(JsValue)>)
}

pub fn add_state_changed_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
    where F: 'static,
    F: Fn(f64) -> ()
{
    Closure::wrap(Box::new(move | event: JsValue | {
        #[allow(unused_unsafe)]
        let event_data = unsafe {
            js_sys::Reflect::get(&event, &wasm_bindgen::JsValue::from_str("data"))
        };

        if let Ok(event_data) = event_data {
            cb(event_data.as_f64().unwrap());
        }
    }) as Box<dyn FnMut(JsValue)>)
}

pub fn add_quality_changed_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
    where F: 'static,
    F: Fn(String) -> ()
{
    Closure::wrap(Box::new(move | event: JsValue | {
        #[allow(unused_unsafe)]
        let event_data = unsafe {
            js_sys::Reflect::get(&event, &wasm_bindgen::JsValue::from_str("data"))
        };

        if let Ok(event_data) = event_data {
            cb(event_data.as_string().unwrap());
        }
    }) as Box<dyn FnMut(JsValue)>)
}

pub fn add_error_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
    where F: 'static,
    F: Fn(JsValue) -> ()
{
    Closure::wrap(Box::new(move | event: JsValue | {
        cb(event);
    }) as Box<dyn FnMut(JsValue)>)
}

// #[wasm_bindgen]
// pub struct IntervalHandle {
//     interval_id: i32,
//     _closure: Closure<dyn FnMut()>,
// }

// impl Drop for IntervalHandle {
//     fn drop(&mut self) {
//         #[allow(unused_unsafe)]
//         unsafe { clear_interval(self.interval_id); }
//     }
// }
//
// #[wasm_bindgen]
// pub fn run() -> IntervalHandle {
//     // First up we use `Closure::wrap` to wrap up a Rust closure and create
//     // a JS closure.
//     let cb = Closure::wrap(Box::new(|| {
//         #[allow(unused_unsafe)]
//         unsafe { gloo::console::log!("interval elapsed!"); }
//     }) as Box<dyn FnMut()>);

//     // Next we pass this via reference to the `setInterval` function, and
//     // `setInterval` gets a handle to the corresponding JS closure.
//     #[allow(unused_unsafe)]
//     let interval_id = unsafe { set_interval(&cb, 1_000) };

//     // If we were to drop `cb` here it would cause an exception to be raised
//     // whenever the interval elapses. Instead we *return* our handle back to JS
//     // so JS can decide when to cancel the interval and deallocate the closure.
//     IntervalHandle {
//         interval_id,
//         _closure: cb,
//     }
// }
