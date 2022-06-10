import { initYtApi, YoutubePlayer, PlayerEventName, PlayerOptions } from '../../pkg/youtube_player_api';

declare global {
  interface Window {
    ytPlayer?: Promise<YoutubePlayer>;
  }
}

/**
 * Load and initialize library.
 *
 * The `default` import is an initialization function which
 * will "boot" the module and make it ready to use. Currently browsers
 * don't support natively imported WebAssembly as an ES module.
 *
 * @link https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html
 */
(async () => {
  // load and initialize youtube player api
  await initYtApi();
  console.log('Module successfully initialized');

  try {
    const ytPlayer = await YoutubePlayer.create('yt-player', {
      videoId: 'cE0wfjsybIQ',
      width: 640,
      height: 360,
      playerVars: {
        autoplay: 0,
        controls: 1,
      },
      events: {
        [PlayerEventName.READY]: () => {
          console.log('Preserve custom player onReady() handler.');
        },
      }
    } as PlayerOptions);

    // set player in global namespace to control it via buttons
    window.ytPlayer = ytPlayer;

    ytPlayer.on(PlayerEventName.STATE_CHANGE, (event: CustomEvent) => {
      console.log('onStateChange', event);
    });

    ytPlayer.on(PlayerEventName.PLAYBACK_QUALITY_CHANGE, (event: CustomEvent) => {
      console.log('onPlaybackQualityChange', event);
    });

    // ytPlayer.playVideo();
  } catch (_err) {
    console.error('Could not bind player!');
  }
})();
