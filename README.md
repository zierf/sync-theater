# SyncTheater.rs


## Yew Frontend

Frontend files are put in folder `frontend/dist` while serving or after a build.

### Prerequisites

Install Trunk to serve and build yew frontend.
```SH
cargo install trunk
```

### Serve frontend

```SH
trunk serve
```

### Build release files

```SH
trunk build --release
```

### Clean build files

Remove dist folder.
```SH
trunk clean
```

Also remove remove libraries.
```SH
cargo clean
```

## Youtube Player API

Build module *.wasm*, JS bindings and TS typings are put in folder `youtube-player-api/pkg`.

### Prerequisites

Install `wasm-pack` to build *.wasm* library for usage outside of yew frontend.
```SH
cargo install wasm-pack
```

Enable `cargo-watch` to rebuild *.wasm* library during development.
```SH
cargo install cargo-watch
```

Use `basic-http-server` to serve local demo HTML page.
```SH
cargo install basic-http-server
```

### Build library for browser

```SH
wasm-pack build --out-name youtube-player-api --target web youtube-player-api
```

### Watch file changes for autmatic library rebuilds

Release version
```SH
cargo watch --no-gitignore -C "./youtube-player-api" -i ".gitignore" -i "pkg" -s "wasm-pack build --out-name youtube-player-api --target web"
```

Debug version  
Feature `std` enables stack traces from Rust for errors in browser console.
Enable it to replace cryptic error `RuntimeError: unreachable executed` with a proper strack trace.
```SH
cargo watch --no-gitignore -C "./youtube-player-api" -i ".gitignore" -i "pkg" -s "wasm-pack build --out-name youtube-player-api --target web --features=std --dev"
```

### Clean build files
```SH
rm -r ./youtube-player-api/pkg
```

### Serve local demo HTML page

```SH
basic-http-server --addr "127.0.0.1:4000" ./youtube-player-api
```
