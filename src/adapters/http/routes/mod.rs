//! Composición de rutas HTTP.
//!
//! Cada submódulo debe exponer el router de un área funcional. Este módulo
//! centraliza el montaje bajo `/api` desde la infraestructura.

/// Endpoints técnicos de salud y disponibilidad.
pub mod health;

use axum::Router;

use crate::adapters::http::app_state::AppState;

/// Construye el router HTTP de todas las rutas versionadas por el prefijo `/api`.
pub fn router() -> Router<AppState> {
    Router::new().nest("/health", health::router())
}
