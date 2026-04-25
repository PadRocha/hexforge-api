use mongodb::{Client, Database, bson::doc, options::ClientOptions};
use thiserror::Error;
use tracing::info;

use crate::infra::config::MongoConfig;

#[derive(Debug, Error)]
pub enum MongoInitError {
    #[error("failed to parse MongoDB connection options")]
    ParseOptions {
        #[source]
        source: mongodb::error::Error,
    },

    #[error("failed to create MongoDB client")]
    CreateClient {
        #[source]
        source: mongodb::error::Error,
    },

    #[error("failed to connect to MongoDB")]
    Ping {
        #[source]
        source: mongodb::error::Error,
    },
}

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
