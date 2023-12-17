use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ctx {
    user_id: usize,
    username: String,
}

impl Ctx {
    pub fn new(user_id: usize, username: String) -> Self {
        Self { user_id, username }
    }
}

impl Ctx {
    pub fn user_id(&self) -> usize {
        self.user_id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
}
