use tracing::info;

mod archive;
mod config;
mod context;
mod controllers;
mod error;
mod request;
mod routes;

fn main() -> Result<(), String> {
    let traces = std::env::var("RUST_LOG").unwrap_or_else(|_| "r5ai=trace".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(traces)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let config = config::get_config();
    let port = config.port;

    info!("Listening on port {}", port);

    let server = warp::serve(routes::get_routes()).run(([127, 0, 0, 1], port));
    let mut rt = tokio::runtime::Runtime::new().map_err(|_| "Error on tokio runtime".to_owned())?;
    rt.block_on(server);
    Ok(())
}
