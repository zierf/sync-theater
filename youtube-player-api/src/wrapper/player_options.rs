use alloc::string::String;
use js_sys::Object;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerVars {
    pub autoplay: Option<i32>,
    pub controls: Option<i32>,
}

impl PlayerVars {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn autoplay(mut self, autoplay: i32) -> Self {
        self.autoplay = Some(autoplay);
        self
    }

    pub fn controls(mut self, controls: i32) -> Self {
        self.controls = Some(controls);
        self
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerOptions {
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    #[serde(rename = "playerVars")]
    pub player_vars: Option<PlayerVars>,
}

impl PlayerOptions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn video_id(mut self, video_id: String) -> Self {
        self.video_id = Some(video_id);
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn player_vars(mut self, player_vars: PlayerVars) -> Self {
        self.player_vars = Some(player_vars);
        self
    }
}

impl From<Object> for PlayerOptions {
    fn from(options: Object) -> Self {
        let js_value = JsValue::from(&options);

        let player_options: Result<PlayerOptions, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(js_value);

        player_options.unwrap()
    }
}

impl From<PlayerOptions> for Object {
    fn from(options: PlayerOptions) -> Self {
        let options: JsValue = options.into();
        options.unchecked_into::<js_sys::Object>()
    }
}

impl From<PlayerOptions> for JsValue {
    fn from(options: PlayerOptions) -> Self {
        let options = serde_wasm_bindgen::to_value(&options);
        options.unwrap()
    }
}
