
var apiResolve, apiReject;
var ytApiReady = new Promise(function (resolve, reject) {
	apiResolve = resolve;
	apiReject = reject;
});

var playerResolve, playerReject;
var ytPlayerReady = new Promise(function (resolve, reject) {
	playerResolve = resolve;
	playerReject = reject;
});

function onYouTubePlayerAPIReady() {
	apiResolve();
}

// (function() {
//   // Load the IFrame Player API code asynchronously.
//   var tag = document.createElement('script');
//   tag.src = "https://www.youtube.com/player_api";
//   var firstScriptTag = document.getElementsByTagName('script')[0];
//   firstScriptTag.parentNode.insertBefore(tag, firstScriptTag);
// })();

function bindPlayer() {
	ytApiReady.then(function () {
		// Replace the 'yt-player' element with an <iframe> and
		// YouTube player after the API code downloads.
		playerInstance = new YT.Player('yt-player', {
			playerVars: {
				'autoplay': 1,
				'controls': 0,
				'fullscreen': 1,
			},
			events: {
				'onReady': onPlayerReady,
				'onPlaybackQualityChange': onPlayerPlaybackQualityChange,
				'onStateChange': onPlayerStateChange,
				'onError': onPlayerError
			}
		});

		return playerInstance;
	});
}

// 4. The API will call this function when the video player is ready.
function onPlayerReady(event) {
	//event.target.playVideo();
	playerResolve(event.target);
}

// 5. The API calls this function when the player's state changes.
//    The function indicates that when playing a video (state=1),
//    the player should play for six seconds and then stop.
var done = false;
function onPlayerStateChange(event) {
	// UNSTARTED: -1, ENDED: 0, PLAYING: 1, PAUSED: 2, BUFFERING: 3, CUED: 5
	console.log('player state', event.data);
	// if (event.data == YT.PlayerState.PLAYING) {}
}

function onPlayerPlaybackQualityChange(event) {
	console.log('player quality changed', event.data);
}

function onPlayerError(event) {
	console.log('player error', event);
}

async function changeVideo(videoId) {
	let player = await ytPlayerReady;
	player.cueVideoById(videoId);
}

async function playVideo() {
	// ytPlayerReady.then(player => player.playVideo());
	let player = await ytPlayerReady;
	player.playVideo();
}

async function pauseVideo() {
	let player = await ytPlayerReady;
	player.pauseVideo();
}

async function stopVideo() {
	let player = await ytPlayerReady;
	player.stopVideo();
	//player.clearVideo();
}

async function seekVideo(seconds) {
	let player = await ytPlayerReady;
	player.seekTo(seconds, true);
}

async function getPlayerState() {
	// YT.PlayerState.[STATE]: UNSTARTED: -1, ENDED: 0, PLAYING: 1, PAUSED: 2, BUFFERING: 3, CUED: 5
	let player = await ytPlayerReady;
	return player.getPlayerState();
}

async function getPlayerTime() {
	let player = await ytPlayerReady;
	return player.getCurrentTime();
}

async function getVideoDuration() {
	let player = await ytPlayerReady;
	return player.getDuration();
}
