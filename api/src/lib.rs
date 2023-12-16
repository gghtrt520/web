use std::time::SystemTime;

use axum::{Router, routing::post};
use jsonwebtoken::{EncodingKey, DecodingKey};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use crate::handlers::login::login;

pub mod middleware;
pub mod handlers;

pub fn routes() ->Router{
    Router::new().route("/login", post(login))
}




fn get_epoch() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
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
    expired: usize,
}