use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "YtGlobal")]
    pub type YtGlobalObject;

    #[wasm_bindgen(typescript_type = "YoutubePlayerInstance")]
    #[derive(Debug)]
    pub type PlayerInstance;

    #[wasm_bindgen(method, js_name = addEventListener)]
    pub fn add_event_listener(this: &PlayerInstance, event: JsString, listener: JsValue);

    // TODO removeEventListener(...) isn't working as expected (see https://issuetracker.google.com/issues/35175764)
    // Since the wrapper uses every handler only used once, it's not too important at the moment.
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
const PLAYER_STATE: &'static str = r#"
export const enum PlayerState {
  UNSTARTED = -1,
  ENDED = 0,
  PLAYING = 1,
  PAUSED = 2,
  BUFFERING = 3,
  CUED = 5,
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const PLAYER_VARS: &'static str = r#"
export interface PlayerVars {
  autoplay?: 0 | 1;
  controls?: 0 | 1;
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const PLAYER_OPTIONS: &'static str = r#"
export interface PlayerOptions {
  videoId?: string;
  width?: number;
  height?: number;
  playerVars?: PlayerVars;
  events?: PlayerEvents;
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const PLAYER_EVENT_NAME: &'static str = r#"
export const enum PlayerEventName {
  READY = 'ready',
  ERROR = 'error',
  STATE_CHANGE = 'stateChange',
  PLAYBACK_QUALITY_CHANGE = 'playbackQualityChange',
  PLAYBACK_RATE_CHANGE = 'playbackRateChange',
  API_CHANGE = 'apiChange',
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const PLAYER_EVENTS: &'static str = r#"
export interface PlayerEvents {
  [key: PlayerEventName | string]: (event?: CustomEvent) => void;
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const YouTube_PLAYER_EVENTS: &'static str = r#"
interface YouTubePlayerEvents {
  onReady?: () => void;
  onError?: () => void;
  onStateChange?: () => void;
  onPlaybackQualityChange?: () => void;
  onPlaybackRateChange?: () => void;
  onApiChange?: () => void;
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const YT_Global: &'static str = r#"
interface YouTubePlayerConstructor {
  new (playerId: string, options?: PlayerOptions): YoutubePlayerInstance;
}

interface YtGlobal {
  Player: YouTubePlayerConstructor;
  PlayerState: {
    [key in keyof typeof PlayerState]: number;
  };
}

declare global {
  var YT: YtGlobal;
}
"#;

// https://github.com/gajus/youtube-player/issues/78
#[wasm_bindgen(typescript_custom_section)]
const YOUTUBE_PLAYER: &'static str = r#"
export interface YoutubePlayerInstance {
  addEventListener(event: string, listener: (event: CustomEvent) => void): void;
  removeEventListener(event: string, listener: (event: CustomEvent) => void): void;

  destroy(): void;
  getAvailablePlaybackRates(): ReadonlyArray<number>;
  getAvailableQualityLevels(): ReadonlyArray<string>;
  getCurrentTime(): number;
  getDuration(): number;
  getIframe(): HTMLIFrameElement;
  getOption(module: string, option: string): any;
  getOptions(): string[];
  getOptions(module: string): object;
  setOption(module: string, option: string, value: any): void;
  setOptions(): void;
  cuePlaylist(
    playlist: string | ReadonlyArray<string>,
    index?: number,
    startSeconds?: number,
    suggestedQuality?: string,
  ): void;
  cuePlaylist(playlist: {
    listType: string,
    list?: string | undefined,
    index?: number | undefined,
    startSeconds?: number | undefined,
    suggestedQuality?: string | undefined,
  }): void;
  loadPlaylist(
    playlist: string | ReadonlyArray<string>,
    index?: number,
    startSeconds?: number,
    suggestedQuality?: string,
  ): void;
  loadPlaylist(playlist: {
    listType: string,
    list?: string | undefined,
    index?: number | undefined,
    startSeconds?: number | undefined,
    suggestedQuality?: string | undefined,
  }): void;
  getPlaylist(): ReadonlyArray<string>;
  getPlaylistIndex(): number;
  getPlaybackQuality(): string;
  getPlaybackRate(): number;
  getPlayerState(): PlayerState;
  getVideoEmbedCode(): string;
  getVideoLoadedFraction(): number;
  getVideoUrl(): string;
  getVolume(): number;
  cueVideoById(videoId: string, startSeconds?: number, suggestedQuality?: string): void;
  cueVideoById(video: {
    videoId: string,
    startSeconds?: number | undefined,
    endSeconds?: number | undefined,
    suggestedQuality?: string | undefined,
  }): void;
  cueVideoByUrl(mediaContentUrl: string, startSeconds?: number, suggestedQuality?: string): void;
  cueVideoByUrl(video: {
    mediaContentUrl: string,
    startSeconds?: number | undefined,
    endSeconds?: number | undefined,
    suggestedQuality?: string | undefined,
  }): void;
  loadVideoByUrl(mediaContentUrl: string, startSeconds?: number, suggestedQuality?: string): void;
  loadVideoByUrl(video: {
    mediaContentUrl: string,
    startSeconds?: number | undefined,
    endSeconds?: number | undefined,
    suggestedQuality?: string | undefined,
  }): void;
  loadVideoById(videoId: string, startSeconds?: number, suggestedQuality?: string): void;
  loadVideoById(video: {
    videoId: string,
    startSeconds?: number | undefined,
    endSeconds?: number | undefined,
    suggestedQuality?: string | undefined,
  }): void;
  isMuted(): boolean;
  mute(): void;
  nextVideo(): void;
  pauseVideo(): void;
  playVideo(): void;
  playVideoAt(index: number): void;
  previousVideo(): void;
  seekTo(seconds: number, allowSeekAhead: boolean): void;
  setLoop(loopPlaylists: boolean): void;
  setPlaybackQuality(suggestedQuality: string): void;
  setPlaybackRate(suggestedRate: number): void;
  setShuffle(shufflePlaylist: boolean): void;
  setSize(width: number, height: number): object;
  setVolume(volume: number): void;
  stopVideo(): void;
  unMute(): void;
}
"#;
