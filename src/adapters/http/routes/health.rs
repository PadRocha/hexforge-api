//! Endpoints técnicos de salud.
//!
//! Estos handlers permiten distinguir entre proceso HTTP vivo y disponibilidad
//! de dependencias externas como MongoDB.

use axum::{Json, Router, extract::State, routing::get};
use mongodb::bson::doc;
use serde::Serialize;

use crate::{AppError, adapters::http::app_state::AppState};

/// Construye las rutas:
///
/// - `GET /api/health`
/// - `GET /api/health/ready`
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(health))
        .route("/ready", get(readiness))
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

/// Atiende `GET /api/health`.
///
/// Responde si el proceso HTTP está vivo sin consultar dependencias externas.
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

/// Atiende `GET /api/health/ready`.
///
/// Ejecuta un `ping` contra MongoDB para verificar que la dependencia de
/// persistencia está disponible.
///
/// # Errors
///
/// Devuelve [`AppError::PersistenceUnavailable`] si MongoDB no responde
/// correctamente al comando `ping`.
///
/// # Side effects
///
/// Realiza I/O de red contra MongoDB.
async fn readiness(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    state
        .database
        .run_command(doc! { "ping": 1 })
        .await
        .map_err(|source| {
            tracing::error!(error = %source, "MongoDB readiness check failed");
            AppError::PersistenceUnavailable
        })?;

    Ok(Json(HealthResponse { status: "ready" }))
}
