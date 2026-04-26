use std::fs::File;
use std::sync::Arc;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::http::app_state::AppState,
    infra::{config::AppConfig, db::init_db, startup_error::StartupError},
};

/// Inicializa las dependencias compartidas por la aplicación HTTP.
///
/// Actualmente crea la conexión lógica a MongoDB y compone el [`AppState`] que
/// consumen los handlers.
///
/// # Errors
///
/// Devuelve [`StartupError::Mongo`] si MongoDB no puede inicializarse.
///
/// # Side effects
///
/// Realiza I/O de red contra MongoDB durante la verificación inicial.
pub async fn init_app_state(config: Arc<AppConfig>) -> Result<AppState, StartupError> {
    let database = Arc::new(init_db(&config.mongo).await?);
    Ok(AppState::new(config, database))
}

/// Inicializa el subscriber global de tracing.
///
/// Configura una capa legible para consola y una capa JSON escrita en `app.log`.
/// El filtro se toma de `RUST_LOG` o usa `apihules=debug,tower_http=debug`.
///
/// # Errors
///
/// Devuelve [`StartupError::CreateLogFile`] si `app.log` no puede crearse y
/// [`StartupError::InitTracing`] si ya existe un subscriber global o falla la
/// inicialización.
///
/// # Side effects
///
/// Crea o trunca `app.log` en el directorio de trabajo e instala un subscriber
/// global para todo el proceso.
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
