use std::io;

use axum::http;
use thiserror::Error;
use tracing_subscriber::util::TryInitError;

use crate::infra::{config::ConfigError, db::MongoInitError};

#[derive(Debug, Error)]
pub enum StartupError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Mongo(#[from] MongoInitError),

    #[error("failed to create log file")]
    CreateLogFile {
        #[source]
        source: io::Error,
    },

    #[error("failed to initialize tracing subscriber")]
    InitTracing {
        #[source]
        source: TryInitError,
    },

    #[error("invalid CORS origin configuration")]
    InvalidCorsOrigin {
        #[source]
        source: http::header::InvalidHeaderValue,
    },

    #[error("failed to bind HTTP server to {address}")]
    BindServer {
        address: String,
        #[source]
        source: io::Error,
    },

    #[error("failed to read bound HTTP server address")]
    LocalAddress {
        #[source]
        source: io::Error,
    },

    #[error("HTTP server exited unexpectedly")]
    ServeHttp {
        #[source]
        source: io::Error,
    },
}
