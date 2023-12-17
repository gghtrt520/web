use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{get_epoch, Claims, KEYS};

pub async fn login(Json(paylod): Json<Palyload>) -> impl IntoResponse {
    tracing::info!(
        "handler=login, username={},password={}",
        paylod.username,
        paylod.password
    );
    let exp = get_epoch();
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            user_id: 1,
            username: paylod.username,
            exp: exp,
        },
        &KEYS.encoding,
    )
    .unwrap();
    (StatusCode::OK, Json(Token { token, exp }))
}

#[derive(Debug, Deserialize)]
pub struct Palyload {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct Token {
    token: String,
    exp: usize,
}
