# SyncTheater.rs


## Yew Frontend

Frontend files are put in folder `frontend/dist` while serving or after a build.

### Prerequisites

Install Trunk to serve and build yew frontend.
```SH
cargo install trunk
```

### Serve frontend

Serve frontend with a local development server under `http://localhost:8080`.

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

Most commands are usable from project root folder, except building and serving the example HTML page.

### Prerequisites

Install `wasm-pack` to build *.wasm* library for usage outside of yew frontend.
```SH
cargo install wasm-pack
```

Enable `cargo-watch` to rebuild *.wasm* library during development.
```SH
cargo install cargo-watch
```

Install `NodeJS` to build and serve the example HTML page.

### Build library for browser

Checkout [Deploying Rust and WebAssembly](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html) to see different deployment options.
To load the library directly in the browser, compile with the following command and follow the description [Without a Bundler](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html) to load it in your page.

```SH
wasm-pack build --target web youtube-player-api
```

### Watch file changes for autmatic library rebuilds

**Release version**
```SH
cargo watch --no-gitignore -C "./youtube-player-api" -i ".gitignore" -i "pkg" -s "wasm-pack build --target web"
```

**Debug version**  
Feature `std` enables stack traces from Rust for errors in browser console.
Enable it to replace cryptic error `RuntimeError: unreachable executed` with a proper strack trace.
```SH
cargo watch --no-gitignore -C "./youtube-player-api" -i ".gitignore" -i "pkg" -s "wasm-pack build --target web --features=std --dev"
```

### Clean build files

```SH
rm -r ./youtube-player-api/pkg
```

### Example HTML page

The example page has to be build from it's own folder.

```SH
cd ./youtube-player-api/examples/typescript
```

Make sure to install the necessary development dependencies before running the first build.

```SH
npm install
```

Build example page in `./dist` folder.

```SH
npm run build
```

The example can also be served with a local development server under address `http://localhost:4000`.

```SH
npm run serve
```

Clean build files.

```SH
npm run clean
```
