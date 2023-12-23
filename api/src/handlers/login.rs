use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use error::Error;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use store::entity::user::Entity;
use store::{entity::user, Store};

use crate::{get_epoch, Claims, KEYS};

pub async fn login(State(store): State<Store>, Json(paylod): Json<Palyload>) -> impl IntoResponse {
    let user = Entity::find()
        .filter(
            user::Column::Username
                .eq(paylod.username)
                .add(user::Column::Password.eq(paylod.password)),
        )
        .one(&store.db)
        .await
        .map_err(|_| Error::DatabaseConnectionError)
        .unwrap()
        .ok_or(Error::NotFound)
        .unwrap();

    tracing::info!(
        "handler=login, username={},password={}",
        user.username,
        user.password
    );
    let exp = get_epoch();
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            user_id: user.id as usize,
            username: user.username,
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
