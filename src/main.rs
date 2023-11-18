use anyhow::Result;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use std::net::SocketAddr;
use tracing::info;

mod auth;
mod router;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting reverse proxy service");
    let router = router::setup_router().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
