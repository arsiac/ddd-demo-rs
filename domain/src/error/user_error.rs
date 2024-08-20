use super::ErrorTrait;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("User '{0}' not found")]
    NotFound(String),
    #[error("The email address '{0}' has been used by another user")]
    DuplicateEmail(String),
    #[error("Invalid email address '{0}'")]
    InvalidEmail(String),
}

impl ErrorTrait for UserError {
    fn code(&self) -> u32 {
        match self {
            UserError::NotFound(_) => 100_001,
            UserError::DuplicateEmail(_) => 100_002,
            UserError::InvalidEmail(_) => 100_003,
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }
}
