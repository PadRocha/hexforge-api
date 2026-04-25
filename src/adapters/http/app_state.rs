use std::sync::Arc;

use axum::extract::FromRef;
use mongodb::Database;

use crate::infra::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub database: Arc<Database>,
}

impl AppState {
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
