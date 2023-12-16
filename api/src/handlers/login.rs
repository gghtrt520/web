use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{get_epoch, Claims, KEYS};

pub async fn login(Json(paylod): Json<Palyload>) -> impl IntoResponse {
    tracing::info!(
        "handler=login, username={},password={}",
        paylod.username,
        paylod.password
    );
    let expired = get_epoch();
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            user_id: 1,
            username: "ba@.com".to_string(),
            expired: expired,
        },
        &KEYS.encoding,
    )
    .unwrap();
    (StatusCode::OK, Json(Token { token, expired }))
}

#[derive(Debug, Deserialize)]
pub struct Palyload {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct Token {
    token: String,
    expired: usize,
}
