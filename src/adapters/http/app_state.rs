use std::sync::Arc;

use axum::extract::FromRef;
use mongodb::Database;

use crate::infra::config::AppConfig;

/// Estado compartido por los handlers HTTP.
///
/// Agrupa dependencias inicializadas en el arranque para que Axum pueda
/// inyectarlas mediante `State` o `FromRef` sin acoplar cada handler al proceso
/// de inicialización.
#[derive(Clone)]
pub struct AppState {
    /// Configuración efectiva de la aplicación.
    pub config: Arc<AppConfig>,
    /// Conexión lógica a la base de datos MongoDB configurada.
    pub database: Arc<Database>,
}

impl AppState {
    /// Crea el estado raíz de la aplicación HTTP.
    pub fn new(config: Arc<AppConfig>, database: Arc<Database>) -> Self {
        Self { config, database }
    }
}

impl FromRef<AppState> for Arc<AppConfig> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.config.clone()
    }
}

impl FromRef<AppState> for Arc<Database> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.database.clone()
    }
}
