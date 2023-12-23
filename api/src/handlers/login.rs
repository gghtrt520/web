use axum::{extract::State, Json};
use error::{Error, Result};
use serde::{Deserialize, Serialize};
use store::entity::user::Entity;
use store::{entity::user, Store};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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
        .await;
    let user = user.unwrap().ok_or(Error::NotFound).unwrap();
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
