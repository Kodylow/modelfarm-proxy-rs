// src/router.rs

use axum::{middleware, Router};
use reverse_proxy_service::TrimPrefix;
use tracing::info;

use anyhow::Result;
use tower_http::cors::{Any, CorsLayer};

use crate::auth::add_auth;

pub fn setup_router() -> Result<Router> {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .expose_headers(Any);
    let host =
        reverse_proxy_service::builder_https("https://production-modelfarm.replit.com/").unwrap();
    let service = host.build(TrimPrefix("modelfarm"));
    let router = Router::new()
        .nest_service("/modelfarm", service)
        .layer(middleware::from_fn(add_auth))
        .layer(cors);

    info!("Reverse Proxy setup for /modelfarm");

    Ok(router)
}
