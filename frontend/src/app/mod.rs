use std::sync::Arc;

use gloo::{console::log, timers::future::TimeoutFuture};
use wasm_bindgen::{prelude::Closure, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;
use youtube_player_api::{init_yt_api, PlayerEvents, PlayerOptions, PlayerVars, YtPlayer};

pub enum Msg {
    ActivatePlayer,
    BindPlayer,
    PlayVideo,
    PauseVideo,
    StopVideo,
    ChangeVideo(String),
    ReadPlayerState,
}

#[derive(Debug, Default, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

pub struct App {
    active: bool,
    player_instance: Arc<Option<YtPlayer>>,
    // on_ready: Closure<dyn FnMut(JsValue)>,
    // on_state_change: Closure<dyn FnMut(JsValue)>,
    // on_quality_change: Closure<dyn FnMut(JsValue)>,
    // on_error: Closure<dyn FnMut(JsValue)>,
}

impl App {
    fn get_player_instance(&self) -> Option<&YtPlayer> {
        self.player_instance.as_ref().as_ref()
    }

    fn run_player<F>(&self, cb: F)
    where
        F: FnOnce(&YtPlayer) -> (),
    {
        let player_instance_option = self.get_player_instance();
        player_instance_option.map(cb);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        // load Youtube Player API scripts
        let _initialized = JsFuture::from(init_yt_api());

        // let handle_ready = self::add_ready_event_handler(|_player_instance: PlayerInstance| {
        //     log!("player ready");
        // });

        // let handle_state_change = self::add_state_changed_event_handler(|state: f64| {
        //     // YT.PlayerState.[STATE]: UNSTARTED: -1, ENDED: 0, PLAYING: 1, PAUSED: 2, BUFFERING: 3, CUED: 5
        //     log!(format!("player state changed: \"{}\"", state));
        // });

        // let handle_quality_change = self::add_quality_changed_event_handler(|quality: String| {
        //     log!(format!("player quality changed: \"{}\"", quality));
        // });

        // let register_error = self::add_error_event_handler(|error: JsValue| {
        //     log!(format!("player error: \"{:?}\"", error));
        // });

        Self {
            active: false,
            player_instance: Arc::new(None),
            // on_ready: handle_ready,
            // on_state_change: handle_state_change,
            // on_quality_change: handle_quality_change,
            // on_error: register_error,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::ActivatePlayer => {
                // Enforce a site interaction, otherwise autoplay could potentially not work.
                // At least one site interaction is needed to enable automatic player control
                // for Browsers blocking autoplay by default.
                self.active = true;

                let cb = link.callback(|_| Msg::BindPlayer);

                // wait for another loop that YT-iFrame can be rendered in page
                spawn_local(async move {
                    TimeoutFuture::new(0).await;
                    cb.emit(0);
                });
            }
            Msg::BindPlayer => {
                // retrieve player instance, YT-iFrame must already exist
                let player_vars = PlayerVars::new().autoplay(1).controls(0);

                let player_options = PlayerOptions::new()
                    .video_id("cE0wfjsybIQ".to_owned())
                    .width(640)
                    .height(360)
                    .player_vars(player_vars);

                let player_instance = YtPlayer::new(&"yt-player".to_owned(), player_options.into());
                self.player_instance = Arc::new(Some(player_instance));

                // FIXME handle events, currently not called
                let quality_change_handler =
                    self::add_quality_changed_event_handler(|quality: String| {
                        log!(format!("player quality changed: \"{}\"", quality));
                    });

                self.player_instance.as_ref().as_ref().unwrap().on(
                    PlayerEvents::get_handler_name(PlayerEvents::PLAYBACK_QUALITY_CHANGE.into()),
                    quality_change_handler.into_js_value(),
                );

                // bind player events
                // self.run_player(|instance| {
                //     instance.on("ready".to_owned(), &self.on_ready);
                //     instance.on("stateChange".to_owned(), &self.on_state_change);
                //     instance.on("playbackQualityChange".to_owned(), &self.on_quality_change);
                //     instance.on("error".to_owned(), &self.on_error);
                // });
            }
            Msg::PlayVideo => {
                self.run_player(|instance| instance.play_video());
            }
            Msg::PauseVideo => {
                self.run_player(|instance| instance.pause_video());
            }
            Msg::StopVideo => {
                self.run_player(|instance| instance.stop_video());
            }
            Msg::ChangeVideo(video_id) => {
                self.run_player(|instance| instance.change_video(video_id));
            }
            Msg::ReadPlayerState => {
                self.run_player(|instance| {
                    let state = instance.get_player_state();
                    log!(format!("current player state: \"{}\"", state));
                });

                return false;
            }
        }

        // the value has changed so we need to
        // re-render for it to appear on the page
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let props = ctx.props();

        let frame_url =
            "http://www.youtube.com/embed/cE0wfjsybIQ?enablejsapi=1&autoplay=1&controls=1";

        html! {
            <>
                <h1>{&props.name}</h1>

                if !self.active {
                    <button onclick={link.callback(|_| Msg::ActivatePlayer)}>{"Activate Player"}</button>
                } else {
                    <iframe id="yt-player" type="text/html"
                        width="640"
                        height="360"
                        frameborder="0"
                        src={frame_url}
                        sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
                        allow="autoplay"
                        allowfullscreen=true>
                    </iframe>
                }
                <br />
                <br />

                <button onclick={link.callback(|_| Msg::PlayVideo)}>{"play video"}</button>
                <button onclick={link.callback(|_| Msg::PauseVideo)}>{"pause video"}</button>
                <button onclick={link.callback(|_| Msg::StopVideo)}>{"stop video"}</button>
                <button onclick={link.callback(|_| Msg::ChangeVideo("bS4Q-WWyl3Q".to_owned()))}>{"change video"}</button>
                <button onclick={link.callback(|_| Msg::ReadPlayerState)}>{"log player state"}</button>
            </>
        }
    }
}

// fn add_ready_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
// where
//     F: 'static,
//     F: Fn(PlayerInstance) -> (),
// {
//     Closure::wrap(Box::new(move |event: JsValue| {
//         #[allow(unused_unsafe)]
//         let event_target =
//             unsafe { js_sys::Reflect::get(&event, &wasm_bindgen::JsValue::from_str("target")) };

//         // FIXME find a way to ensure it's a proper player instance
//         // let yt_player = event_target.and_then(|event_target| event_target.dyn_into::<PlayerInstance>()).ok();
//         // yt_player.map(|player_instance| cb(player_instance));

//         if let Ok(event_target) = event_target {
//             cb(event_target.unchecked_into::<PlayerInstance>());
//         }
//     }) as Box<dyn FnMut(JsValue)>)
// }

// fn add_state_changed_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
// where
//     F: 'static,
//     F: Fn(f64) -> (),
// {
//     Closure::wrap(Box::new(move |event: JsValue| {
//         #[allow(unused_unsafe)]
//         let event_data =
//             unsafe { js_sys::Reflect::get(&event, &wasm_bindgen::JsValue::from_str("data")) };

//         if let Ok(event_data) = event_data {
//             cb(event_data.as_f64().unwrap());
//         }
//     }) as Box<dyn FnMut(JsValue)>)
// }

fn add_quality_changed_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
where
    F: 'static,
    F: Fn(String) -> (),
{
    Closure::wrap(Box::new(move |event: JsValue| {
        #[allow(unused_unsafe)]
        let event_data =
            unsafe { js_sys::Reflect::get(&event, &wasm_bindgen::JsValue::from_str("data")) };

        if let Ok(event_data) = event_data {
            cb(event_data.as_string().unwrap());
        }
    }) as Box<dyn FnMut(JsValue)>)
}

// fn add_error_event_handler<F>(cb: F) -> Closure<dyn FnMut(JsValue)>
// where
//     F: 'static,
//     F: Fn(JsValue) -> (),
// {
//     Closure::wrap(Box::new(move |event: JsValue| {
//         cb(event);
//     }) as Box<dyn FnMut(JsValue)>)
// }
