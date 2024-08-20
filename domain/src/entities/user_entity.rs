use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Local};

use crate::error::UserError;

const EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

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

pub fn check_email(email: &str) -> crate::error::Result<()> {
    let email_pattern = regex::Regex::new(EMAIL_REGEX).unwrap();
    if !email_pattern.is_match(email) {
        return Err(UserError::InvalidEmail(email.into()).into());
    }
    Ok(())
}

#[derive(Debug, Default, Clone)]
pub struct Email(String);

impl FromStr for Email {
    type Err = crate::error::DomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        check_email(value)?;
        Ok(Email(value.into()))
    }
}

impl TryFrom<String> for Email {
    type Error = crate::error::DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        check_email(&value)?;
        Ok(Email(value))
    }
}

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
