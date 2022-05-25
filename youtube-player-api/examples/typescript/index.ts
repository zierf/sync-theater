import { initYtApi, YoutubePlayer, PlayerOptions, YoutubePlayerInstance } from '../../pkg/youtube_player_api';

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

  setTimeout(async () => {
    const ytPlayer = await YoutubePlayer.create('yt-player', {
      videoId: 'cE0wfjsybIQ',
      width: 640,
      height: 360,
      playerVars: {
        autoplay: 0,
        controls: 1,
      },
      events: {
        onReady: (target: YoutubePlayerInstance) => {
          console.log('Preserve custom player onReady() handler.');
        },
      }
    } as PlayerOptions);

    // set player in global namespace to control it via buttons
    window.ytPlayer = ytPlayer;

    ytPlayer.on('stateChange', (event: CustomEvent) => {
      console.log('onStateChange', event);
    });

    // ytPlayer.on('playbackQualityChange', (event) => {
    // 	console.log('onPlaybackQualityChange', event);
    // });

    // ytPlayer.playVideo();
  }, 1000);
})();
