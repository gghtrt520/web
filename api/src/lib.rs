use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use access::access;
use auth::auth;
use handlers::login::login;
use handlers::user::get_user;
use store::Store;

pub mod access;
pub mod auth;
pub mod ctx;
pub mod handlers;

pub fn routes(store: Store) -> Router {
    Router::new()
        .route("/login", post(login))
        .merge(routes_with_auth(store))
}

pub fn routes_with_auth(store: Store) -> Router {
    Router::new()
        .route("/user", get(get_user))
        .layer(middleware::from_fn(access))
        .layer(middleware::from_fn(auth))
        .with_state(store)
}

fn get_epoch() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3000
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    Keys::new("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM".as_bytes())
});

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Keys {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: usize,
    username: String,
    exp: usize,
}
