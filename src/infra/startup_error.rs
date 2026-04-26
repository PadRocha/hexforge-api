use std::io;

use axum::http;
use thiserror::Error;
use tracing_subscriber::util::TryInitError;

use crate::infra::{config::ConfigError, db::MongoInitError};

/// Error que aborta el arranque del binario.
///
/// Agrupa fallos de configuración, logging, MongoDB, CORS y servidor HTTP para
/// que `main` pueda reportarlos de forma uniforme.
#[derive(Debug, Error)]
pub enum StartupError {
    /// La configuración no pudo cargarse o validarse.
    #[error(transparent)]
    Config(#[from] ConfigError),

    /// MongoDB no pudo inicializarse o no respondió al `ping` de arranque.
    #[error(transparent)]
    Mongo(#[from] MongoInitError),

    /// No se pudo crear el archivo de logs `app.log`.
    #[error("failed to create log file")]
    CreateLogFile {
        /// Error original del sistema de archivos.
        #[source]
        source: io::Error,
    },

    /// No se pudo instalar el subscriber global de tracing.
    #[error("failed to initialize tracing subscriber")]
    InitTracing {
        /// Error original reportado por `tracing-subscriber`.
        #[source]
        source: TryInitError,
    },

    /// El origen CORS configurado no es un header HTTP válido.
    #[error("invalid CORS origin configuration")]
    InvalidCorsOrigin {
        /// Error original al parsear el valor del header.
        #[source]
        source: http::header::InvalidHeaderValue,
    },

    /// El servidor HTTP no pudo escuchar en la dirección configurada.
    #[error("failed to bind HTTP server to {address}")]
    BindServer {
        /// Dirección configurada para el listener HTTP.
        address: String,
        /// Error original del sistema operativo.
        #[source]
        source: io::Error,
    },

    /// No se pudo leer la dirección final asignada al listener HTTP.
    #[error("failed to read bound HTTP server address")]
    LocalAddress {
        /// Error original del sistema operativo.
        #[source]
        source: io::Error,
    },

    /// Axum terminó con un error mientras atendía conexiones HTTP.
    #[error("HTTP server exited unexpectedly")]
    ServeHttp {
        /// Error original reportado por el servidor HTTP.
        #[source]
        source: io::Error,
    },
}
