use axum::{response::IntoResponse, http::StatusCode, Json};

use crate::ctx::Ctx;

pub async fn get_user(ctx:Ctx) -> impl IntoResponse {
    (StatusCode::OK, Json(ctx))
}