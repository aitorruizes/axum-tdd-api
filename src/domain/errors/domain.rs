#[derive(Debug, PartialEq, Eq)]
pub enum DomainError {
    Internal(String),
    PasswordMismatch,
    UserAlreadyExists,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(err) => write!(f, "Something went wrong: {err}"),
            Self::PasswordMismatch => write!(f, "The provided passwords do not match"),
            Self::UserAlreadyExists => {
                write!(f, "An user already exists with the given information")
            }
        }
    }
}

impl std::error::Error for DomainError {}
