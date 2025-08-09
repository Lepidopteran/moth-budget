use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::{extract::MatchedPath, http::Request, Router};
use color_eyre::owo_colors::OwoColorize;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info_span;

mod ui;

/// Start the server
pub async fn serve() {
    let app = Router::new()
        .merge(ui::route())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path = matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    let host = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let addr = SocketAddr::from((host, 3000));
    tracing::info!(
        "Listening on {}{}",
        "http://".underline().blue(),
        addr.underline().blue()
    );

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Wait for a shutdown signal
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
