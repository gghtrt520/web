use axum::{extract::State, Json};
use error::{Error, Result};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use store::entity::user::Entity;
use store::{entity::user, Store};

use crate::{get_epoch, Claims, KEYS};

pub async fn login(
    State(store): State<Store>,
    Json(paylod): Json<Palyload>,
) -> Result<Json<Authcation>> {
    let user = Entity::find()
        .filter(
            user::Column::Username
                .eq(paylod.username)
                .and(user::Column::Password.eq(paylod.password)),
        )
        .one(&store.db)
        .await
        .map_err(|_| Error::DatabaseConnectionError)?;
    let user = user.ok_or(Error::NotFound)?;
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
    .map_err(|_| Error::TokenGenError)?;
    return Ok(Json(Authcation { token, exp }));
}

#[derive(Debug, Deserialize)]
pub struct Palyload {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct Authcation {
    token: String,
    exp: usize,
}
