use chrono::{DateTime, Local};

use super::Email;

#[derive(Debug, Default, Clone)]
pub struct UserEntity {
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub email: Email,
    pub password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug)]
pub struct UserRegisterEntity {
    pub email: Email,
    pub password: String,
}
