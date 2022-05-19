use std::sync::Arc;

use gloo::timers::future::TimeoutFuture;
use wasm_bindgen::{prelude::Closure, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use youtube_player_npm::PlayerInstance;


pub enum Msg {
    ActivatePlayer,
    BindPlayer,
    PlayVideo,
    PauseVideo,
    StopVideo,
    ChangeVideo(String),
    ReadPlayerState,
    DonePlayerState(i32),
}

#[derive(Debug, Default, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

pub struct App {
    active: bool,
    player_instance: Arc<Option<PlayerInstance>>,
    on_ready: Closure<dyn FnMut(JsValue)>,
    on_state_change: Closure<dyn FnMut(JsValue)>,
    on_quality_change: Closure<dyn FnMut(JsValue)>,
    on_error: Closure<dyn FnMut(JsValue)>,
}

impl App {
    fn get_player_instance(&self) -> Option<&PlayerInstance> {
        self.player_instance.as_ref().as_ref()
    }

    fn run_player<F>(&self, cb: F)
        where F: FnOnce(&PlayerInstance) -> ()
    {
        let player_instance_option = self.get_player_instance();
        player_instance_option.map(cb);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let handle_ready = youtube_player_npm::add_ready_event_handler(| _player_instance: PlayerInstance | {
            #[allow(unused_unsafe)]
            unsafe { gloo::console::log!("player ready"); };
            // unsafe { gloo::console::log!(gloo::utils::window()); };
            // unsafe { gloo::console::log!("ready", format!("\"{:?}\"", player_instance)); }
        });

        let handle_state_change = youtube_player_npm::add_state_changed_event_handler(| state: f64 | {
            // YT.PlayerState.[STATE]: UNSTARTED: -1, ENDED: 0, PLAYING: 1, PAUSED: 2, BUFFERING: 3, CUED: 5
            #[allow(unused_unsafe)]
            unsafe { gloo::console::log!("player state changed", format!("\"{}\"", state)); }
        });

        let handle_quality_change = youtube_player_npm::add_quality_changed_event_handler(| quality: String | {
            #[allow(unused_unsafe)]
            unsafe { gloo::console::log!("player quality changed", format!("\"{}\"", quality)); }
        });

        let register_error = youtube_player_npm::add_error_event_handler(| error: JsValue | {
            #[allow(unused_unsafe)]
            unsafe { gloo::console::log!("player error", format!("\"{:?}\"", error)); }
        });

        Self {
            active: false,
            player_instance: Arc::new(None),
            on_ready: handle_ready,
            on_state_change: handle_state_change,
            on_quality_change: handle_quality_change,
            on_error: register_error,
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
            },
            Msg::BindPlayer => {
                // retrieve player instance, YT-iFrame must already exist
                #[allow(unused_unsafe)]
                unsafe {
                    self.player_instance = Arc::new(youtube_player_npm::bind_player_to_instance("yt-player".to_owned()).ok());
                }

                // bind player events
                self.run_player(|instance| {
                    instance.on("ready".to_owned(), &self.on_ready);
                    instance.on("stateChange".to_owned(), &self.on_state_change);
                    instance.on("playbackQualityChange".to_owned(), &self.on_quality_change);
                    instance.on("error".to_owned(), &self.on_error);
                });
            },
            Msg::PlayVideo => {
                self.run_player(|instance| instance.play_video());
            },
            Msg::PauseVideo => {
                self.run_player(|instance| instance.pause_video());
            },
            Msg::StopVideo => {
                self.run_player(|instance| instance.stop_video());
            },
            Msg::ChangeVideo(video_id) => {
                self.run_player(|instance| instance.change_video(video_id));
            },
            Msg::ReadPlayerState => {
                let new_link = link.clone();
                let new_instance = self.player_instance.clone();

                spawn_local(async move {
                    let instance_option = new_instance.as_ref();
                    let player_instance = instance_option.as_ref().unwrap();
                    let state_result = player_instance.get_player_state().await;

                    let _ = state_result.map(move |state| {
                        let new_state: i32 = serde_wasm_bindgen::from_value(state).unwrap();
                        let cb = new_link.callback(move |_| Msg::DonePlayerState(new_state));
                        cb.emit(0);
                    });
                });

                return false;
            },
            Msg::DonePlayerState(player_state) => {
                #[allow(unused_unsafe)]
                unsafe { gloo::console::log!("current player state", format!("\"{}\"", player_state)); };

                return false;
            },
        }

        // the value has changed so we need to
        // re-render for it to appear on the page
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let props = ctx.props();

        let frame_url = "http://www.youtube.com/embed/cE0wfjsybIQ?enablejsapi=1&autoplay=1&controls=1";

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
