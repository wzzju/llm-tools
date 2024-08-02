#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use dotenv::dotenv;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use llm_tools::app::*;
    use llm_tools::fileserv::file_and_error_handler;
    use tower_http::compression::CompressionLayer;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    dotenv().ok();

    // Parse an `EnvFilter` configuration from the `RUST_LOG`
    // environment variable.
    let env_filter = EnvFilter::from_default_env();

    let fmt_layer = fmt::layer()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(true)
        .with_thread_ids(false)
        .with_filter(env_filter);

    tracing_subscriber::registry().with(fmt_layer).init();

    let (shutdown_send, mut shutdown_recv) = tokio::sync::mpsc::unbounded_channel::<()>();

    tokio::spawn(async move {
        // Setting get_configuration(None) means we'll be using cargo-leptos's env values
        // For deployment these variables are:
        // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
        // Alternately a file can be specified such as Some("Cargo.toml")
        // The file would need to be included with the executable when moved to deployment
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);

        // build our application with a route
        let app = Router::new()
            .leptos_routes(&leptos_options, routes, App)
            .fallback(file_and_error_handler)
            .layer(
                CompressionLayer::new()
                    .gzip(true)
                    .br(true)
                    .deflate(true)
                    .quality(tower_http::CompressionLevel::Default),
            )
            .with_state(leptos_options);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        tracing::info!("listening on http://{}", &addr);
        axum::serve(listener, app.into_make_service())
            .await
            .expect("Fail to start the server.");

        tracing::info!("Exiting...");
        shutdown_send.send(()).unwrap();
    });

    tracing::info!("Waiting for Ctrl-C...");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::warn!("Received Ctrl-C signal.");
        },
        _ = shutdown_recv.recv() => {
            tracing::warn!("Received application shutdown signal.");
        },
    }
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use llm_tools::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
