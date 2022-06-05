use alloc::string::String;
use js_sys::Object;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};

// #[wasm_bindgen(typescript_type = "PlayerVars")]
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

// #[wasm_bindgen(typescript_type = "PlayerOptions")]
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

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::borrow::ToOwned;

    #[test]
    fn player_vars_default() {
        // FIXME add all available fields from official youtube api
        let player_vars = PlayerVars::new();

        assert_eq!(None, player_vars.autoplay);
        assert_eq!(None, player_vars.controls);
    }

    #[test]
    fn player_vars_set() {
        // FIXME add all available fields from official youtube api
        let player_vars = PlayerVars::new().autoplay(0).controls(1);

        assert_eq!(Some(0), player_vars.autoplay);
        assert_eq!(Some(1), player_vars.controls);
    }

    #[test]
    fn player_options_default() {
        let player_options = PlayerOptions::new();

        assert_eq!(None, player_options.video_id);
        assert_eq!(None, player_options.width);
        assert_eq!(None, player_options.height);
        assert_eq!(None, player_options.player_vars);
    }

    #[test]
    fn player_options_set() {
        let player_vars = PlayerVars::new();
        let player_options = PlayerOptions::new()
            .video_id("abcdefghij".to_owned())
            .width(640)
            .height(360)
            .player_vars(player_vars.clone());

        assert_eq!(Some("abcdefghij".to_owned()), player_options.video_id);
        assert_eq!(Some(640), player_options.width);
        assert_eq!(Some(360), player_options.height);
        assert_eq!(Some(player_vars), player_options.player_vars);
    }
}
