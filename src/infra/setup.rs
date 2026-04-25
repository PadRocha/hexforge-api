use std::fs::File;
use std::sync::Arc;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::http::app_state::AppState,
    infra::{config::AppConfig, db::init_db, startup_error::StartupError},
};

pub async fn init_app_state(config: Arc<AppConfig>) -> Result<AppState, StartupError> {
    let database = Arc::new(init_db(&config.mongo).await?);
    Ok(AppState::new(config, database))
}

pub fn init_tracing() -> Result<(), StartupError> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "apihules=debug,tower_http=debug".into());

    let console_layer = fmt::layer().with_target(false).with_level(true).pretty();

    let file = File::create("app.log").map_err(|source| StartupError::CreateLogFile { source })?;
    let json_layer = fmt::layer()
        .json()
        .with_writer(file)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(json_layer)
        .try_init()
        .map_err(|source| StartupError::InitTracing { source })?;

    Ok(())
}
