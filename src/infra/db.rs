use mongodb::{Client, Database, bson::doc, options::ClientOptions};
use thiserror::Error;
use tracing::info;

use crate::infra::config::MongoConfig;

/// Errores producidos durante la inicialización de MongoDB.
#[derive(Debug, Error)]
pub enum MongoInitError {
    /// El driver no pudo interpretar la URI o resolver opciones de conexión.
    #[error("failed to parse MongoDB connection options")]
    ParseOptions {
        /// Error original reportado por el driver de MongoDB.
        #[source]
        source: mongodb::error::Error,
    },

    /// El cliente no pudo construirse a partir de las opciones calculadas.
    #[error("failed to create MongoDB client")]
    CreateClient {
        /// Error original reportado por el driver de MongoDB.
        #[source]
        source: mongodb::error::Error,
    },

    /// El cliente fue creado, pero MongoDB no respondió al comando `ping`.
    #[error("failed to connect to MongoDB")]
    Ping {
        /// Error original reportado por el driver de MongoDB.
        #[source]
        source: mongodb::error::Error,
    },
}

/// Inicializa el cliente MongoDB y devuelve la base configurada.
///
/// Aplica las opciones de pool definidas en [`MongoConfig`] y valida la conexión
/// con un comando `ping` antes de entregar la base de datos al resto de la API.
///
/// # Errors
///
/// Devuelve [`MongoInitError`] si la URI no puede parsearse, el cliente no puede
/// construirse o MongoDB no responde al `ping` inicial.
///
/// # Side effects
///
/// Realiza I/O de red contra MongoDB durante el parseo de opciones y el `ping`.
pub async fn init_db(config: &MongoConfig) -> Result<Database, MongoInitError> {
    let mut options = ClientOptions::parse(&config.uri)
        .await
        .map_err(|source| MongoInitError::ParseOptions { source })?;
    options.app_name = Some(config.app_name.clone());

    if let Some(max_pool_size) = config.max_pool_size {
        options.max_pool_size = Some(max_pool_size);
    }

    if let Some(min_pool_size) = config.min_pool_size {
        options.min_pool_size = Some(min_pool_size);
    }

    if let Some(max_idle_time) = config.max_idle_time {
        options.max_idle_time = Some(max_idle_time);
    }

    let client =
        Client::with_options(options).map_err(|source| MongoInitError::CreateClient { source })?;
    let database = client.database(&config.database_name);

    database
        .run_command(doc! { "ping": 1 })
        .await
        .map_err(|source| MongoInitError::Ping { source })?;

    info!("Connected to MongoDB database {}", config.database_name);
    Ok(database)
}
