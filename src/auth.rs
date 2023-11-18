use axum::{
    http::{header::AUTHORIZATION, HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use reqwest::Client;
use serde_json::{json, Value};

use crate::AppError;

pub async fn add_auth<B: std::fmt::Debug>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let bearer_token = get_auth_token().await.unwrap();

    req.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_str(bearer_token.as_str()).unwrap(),
    );

    return Ok(next.run(req).await);
}

async fn get_auth_token() -> anyhow::Result<String> {
    let client = Client::new();
    let response = client
        .post("http://localhost:1105/getIdentityToken")
        .json(&json!({"audience": "modelfarm@replit.com"}))
        .send()
        .await?;

    let content: Value = response.json().await?;
    let identity_token = content["identityToken"]
        .as_str()
        .expect("identityToken not found")
        .to_string();

    Ok(format!("Bearer {}, ", identity_token))
}
