// #[wasm_bindgen(typescript_type = "PlayerState")]
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct PlayerState;

impl PlayerState {
    pub const UNSTARTED: i32 = -1;
    pub const ENDED: i32 = 0;
    pub const PLAYING: i32 = 1;
    pub const PAUSED: i32 = 2;
    pub const BUFFERING: i32 = 3;
    pub const CUED: i32 = 5;
}
