mod user_error;

pub use user_error::UserError;

pub type Result<T> = std::result::Result<T, DomainError>;

pub trait ErrorTrait {
    fn code(&self) -> u32;

    fn message(&self) -> String;
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    User(#[from] UserError),
}

impl ErrorTrait for DomainError {
    fn code(&self) -> u32 {
        match self {
            DomainError::Other(_) => 999_999,
            DomainError::User(ref e) => e.code(),
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }
}
