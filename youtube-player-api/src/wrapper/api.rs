use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type PlayerInstance;

    #[wasm_bindgen(method, js_name = addEventListener)]
    pub fn add_event_listener(this: &PlayerInstance, event: JsString, listener: JsValue);

    // FIXME removeEventListener(...) isn't working as expected
    // #[wasm_bindgen(method, js_name = removeEventListener)]
    // pub fn remove_event_listener(this: &PlayerInstance, event: JsString, listener: JsValue);

    #[wasm_bindgen(method, js_name = playVideo)]
    pub fn play_video(this: &PlayerInstance);

    #[wasm_bindgen(method, js_name = pauseVideo)]
    pub fn pause_video(this: &PlayerInstance);

    #[wasm_bindgen(method, js_name = stopVideo)]
    pub fn stop_video(this: &PlayerInstance);

    #[wasm_bindgen(method, js_name = cueVideoById)]
    pub fn cue_video_by_id(this: &PlayerInstance, video_id: JsString);

    #[wasm_bindgen(method, js_name = getPlayerState)]
    pub fn get_player_state(this: &PlayerInstance) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const YT_API_TYPINGS: &'static str = r#"
export enum YtPlayerState {
  UNSTARTED = -1,
  ENDED = 0,
  PLAYING = 1,
  PAUSED = 2,
  BUFFERING = 3,
  CUED = 5,
}

export interface YtPlayerVars {
  autoplay?: 0 | 1,
  controls?: 0 | 1,
}

interface YtPlayerEvents {
  onReady?: () => void,
  onError?: () => void,
  onStateChange?: () => void,
  onPlaybackQualityChange?: () => void,
  onPlaybackRateChange?: () => void,
  onApiChange?: () => void,
}

export interface YtPlayerOptions {
  videoId?: string,
  width?: number,
  height?: number,
  playerVars?: YtPlayerVars,
  events?: YtPlayerEvents,
}

interface YtGlobalObject {
  Player: (playerId: string, options?: YtPlayerOptions) => void,
  PlayerState: {
    [key in keyof typeof YtPlayerState]: number;
  },
}

declare global {
  var YT: YtGlobalObject;
}
"#;
