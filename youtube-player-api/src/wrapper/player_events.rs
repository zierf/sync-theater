use alloc::{
    format,
    string::{String, ToString},
};

// #[wasm_bindgen(typescript_type = "PlayerEvents")]
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct PlayerEvents;

impl PlayerEvents {
    pub const READY: &'static str = "ready";
    pub const PLAYBACK_QUALITY_CHANGE: &'static str = "playbackQualityChange";
    pub const STATE_CHANGE: &'static str = "stateChange";
    pub const ERROR: &'static str = "error";
    pub const PLAYBACK_RATE_CHANGE: &'static str = "playbackRateChange";
    pub const API_CHANGE: &'static str = "apiChange";

    pub fn get_handler_name(event_name: &str) -> String {
        let mut event_name = event_name.to_string();

        // convert first letter to uppercase
        let handler_name = event_name.remove(0).to_uppercase().to_string() + &event_name;
        // prefix event with on (stateChange => onStateChange)
        let handler_name = format!("{}{}", "on", handler_name);

        handler_name
    }
}
