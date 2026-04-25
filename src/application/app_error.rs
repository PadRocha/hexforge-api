use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppError {
    #[error("Bad request")]
    BadRequest,

    #[error("Validation failed")]
    ValidationFailed,

    #[error("Resource not found")]
    NotFound,

    #[error("Resource already exists")]
    Conflict,

    #[error("Unauthenticated")]
    Unauthenticated,

    #[error("Forbidden")]
    Forbidden,

    #[error("Too many requests")]
    RateLimited,

    #[error("Not implemented")]
    NotImplemented,

    #[error("Bad gateway")]
    BadGateway,

    #[error("Gateway timeout")]
    GatewayTimeout,

    #[error("Persistence unavailable")]
    PersistenceUnavailable,

    #[error("Internal server error")]
    Internal,
}

pub type AppResult<T> = Result<T, AppError>;
