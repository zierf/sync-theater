use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


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
