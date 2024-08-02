<picture>
    <img src="/assets/logo.png" alt="LLM-Tools Logo" height="200">
</picture>

## Prepare

If you don't have `cargo-leptos` installed you can install it with

```sh
cargo install cargo-leptos leptosfmt --locked
```

## Run

```sh
cd llm-tools
make dev
```

By default, you can access your local project at `http://localhost:3000`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary(`llm-tools`) located in `target/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The deployment directory structure should be:
```text
llm-tools
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="llm-tools"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

All commands are summarized as follows:
```sh
#!/usr/bin/env bash

# cargo leptos build --release

rm -rf deploy && mkdir -p deploy
cp target/release/llm-tools deploy/
cp -r target/site/ deploy/

export LEPTOS_OUTPUT_NAME="llm-tools"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"

cd deploy && ./llm-tools && cd -
```
