#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("User '{0}' not found")]
    NotFound(String),
    #[error("The email address '{0}' has been used by another user")]
    DuplicateEmail(String),
}
