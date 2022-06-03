use alloc::{borrow::ToOwned, string::String};

// #[wasm_bindgen(typescript_type = "PlayerEvents")]
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct PlayerEvents;

impl PlayerEvents {
    pub const API_CHANGE: &'static str = "apiChange";
    pub const ERROR: &'static str = "error";
    pub const PLAYBACK_QUALITY_CHANGE: &'static str = "playbackQualityChange";
    pub const PLAYBACK_RATE_CHANGE: &'static str = "playbackRateChange";
    pub const READY: &'static str = "ready";
    pub const STATE_CHANGE: &'static str = "stateChange";

    pub fn get_handler_name(event_name: &str) -> Result<String, &'static str> {
        if event_name.len() == 0 {
            return Err("Event name must not be empty!");
        }

        // prefix event with "on" and convert first letter to uppercase (stateChange => onStateChange)
        let mut handler_name = String::with_capacity(event_name.len() + 2);

        handler_name.push_str("on");
        handler_name.push_str(&event_name[0..1].to_uppercase());
        handler_name.push_str(&event_name[1..event_name.len()]);

        Ok(handler_name)
    }

    pub fn get_event_name(handler_name: &str) -> Result<String, &'static str> {
        if handler_name.len() == 0 {
            return Err("Handler name must not be empty!");
        }

        if handler_name.len() <= 2 || !handler_name.starts_with("on") {
            return Ok(handler_name.to_owned());
        }

        // prefix event with "on" and convert first letter to lowercase (onStateChange => stateChange)
        let mut event_name = String::with_capacity(handler_name.len() - 2);

        event_name.push_str(&handler_name[2..3].to_lowercase());
        event_name.push_str(&handler_name[3..handler_name.len()]);

        Ok(event_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::String;
    // use test::Bencher;

    #[test]
    fn get_handler_name() {
        // enforce typings in first example
        let event_name: &str = PlayerEvents::API_CHANGE;
        let handler_name: Result<String, _> = PlayerEvents::get_handler_name(event_name);

        assert_eq!("onApiChange", handler_name.unwrap());

        // check correct conversion for rest of existing events
        for event in [
            (PlayerEvents::ERROR, "onError"),
            (
                PlayerEvents::PLAYBACK_QUALITY_CHANGE,
                "onPlaybackQualityChange",
            ),
            (PlayerEvents::PLAYBACK_RATE_CHANGE, "onPlaybackRateChange"),
            (PlayerEvents::READY, "onReady"),
            (PlayerEvents::STATE_CHANGE, "onStateChange"),
        ] {
            assert_eq!(event.1, PlayerEvents::get_handler_name(event.0).unwrap());
        }
    }

    #[test]
    fn get_handler_name_empty() {
        let event_name: &str = "";
        let handler_name: Result<String, &'static str> = PlayerEvents::get_handler_name(event_name);

        assert_eq!(true, handler_name.is_err());
    }

    #[test]
    fn get_event_name() {
        // check correct conversion for all existing events
        for event in [
            (PlayerEvents::API_CHANGE, "onApiChange"),
            (PlayerEvents::ERROR, "onError"),
            (
                PlayerEvents::PLAYBACK_QUALITY_CHANGE,
                "onPlaybackQualityChange",
            ),
            (PlayerEvents::PLAYBACK_RATE_CHANGE, "onPlaybackRateChange"),
            (PlayerEvents::READY, "onReady"),
            (PlayerEvents::STATE_CHANGE, "onStateChange"),
        ] {
            assert_eq!(event.0, PlayerEvents::get_event_name(event.1).unwrap());
        }

        // chars after "on" don't have to start uppercase
        assert_eq!("test", PlayerEvents::get_event_name("ontest").unwrap());
    }

    #[test]
    fn get_event_name_keep() {
        // don't convert handler names with less than 2 characters
        assert_eq!("o", PlayerEvents::get_event_name("o").unwrap());
        assert_eq!("on", PlayerEvents::get_event_name("on").unwrap());

        // don't convert handler names not beginning with on
        assert_eq!("noTest", PlayerEvents::get_event_name("noTest").unwrap());

        // only remove lowercase "on"
        assert_eq!("OnTest", PlayerEvents::get_event_name("OnTest").unwrap());
        assert_eq!("oNTest", PlayerEvents::get_event_name("oNTest").unwrap());

        // remove "on" prefix with at least 3 characters
        assert_eq!("t", PlayerEvents::get_event_name("onT").unwrap());
    }

    #[test]
    fn get_event_name_empty() {
        let handler_name: &str = "";
        let event_name: Result<String, &'static str> = PlayerEvents::get_event_name(handler_name);

        assert_eq!(true, event_name.is_err());
    }

    // #[bench]
    // fn bench_with_capacity(b: &mut Bencher) {
    //     let event_name: &str = PlayerEvents::STATE_CHANGE;

    //     b.iter(|| {
    //         let mut handler_name = String::with_capacity(event_name.len() + 2);

    //         handler_name.push_str("on");
    //         handler_name.push_str(&event_name[0..1].to_uppercase());
    //         handler_name.push_str(&event_name[1..event_name.len()]);

    //         handler_name
    //     });
    // }

    // #[bench]
    // fn bench_slices_plus(b: &mut Bencher) {
    //     let event_name: &str = PlayerEvents::STATE_CHANGE;

    //     b.iter(|| {
    //         "on".to_owned() + &event_name[0..1].to_uppercase() + &event_name[1..event_name.len()]
    //     });
    // }

    // #[bench]
    // fn bench_concat(b: &mut Bencher) {
    //     let event_name: &str = PlayerEvents::STATE_CHANGE;

    //     b.iter(|| {
    //         [
    //             "on",
    //             &event_name[0..1].to_uppercase(),
    //             &event_name[1..event_name.len()],
    //         ]
    //         .concat()
    //     });
    // }

    // #[bench]
    // fn bench_join(b: &mut Bencher) {
    //     let event_name: &str = PlayerEvents::STATE_CHANGE;

    //     b.iter(|| {
    //         [
    //             "on",
    //             &event_name[0..1].to_uppercase(),
    //             &event_name[1..event_name.len()],
    //         ]
    //         .join("")
    //     });
    // }

    // #[bench]
    // fn bench_format(b: &mut Bencher) {
    //     let event_name: &str = PlayerEvents::STATE_CHANGE;

    //     b.iter(|| {
    //         format!(
    //             "{}{}{}",
    //             "on",
    //             &event_name[0..1].to_uppercase(),
    //             &event_name[1..event_name.len()]
    //         )
    //     });
    // }

    // #[bench]
    // fn bench_format_args(b: &mut Bencher) {
    //     let event_name: &str = PlayerEvents::STATE_CHANGE;

    //     b.iter(|| {
    //         format_args!(
    //             "{}{}{}",
    //             "on",
    //             &event_name[0..1].to_uppercase(),
    //             &event_name[1..event_name.len()]
    //         )
    //         .to_string()
    //     });
    // }
}
