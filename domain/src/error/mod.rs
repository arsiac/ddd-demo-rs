mod user_error;

pub use user_error::UserError;

pub type Result<T> = std::result::Result<T, DomainError>;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    User(#[from] UserError),
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid email address '{0}'")]
    InvalidEmail(String),
    #[error("Invalid password")]
    InvalidPassword,
}
