use axum::{Json, Router, extract::State, routing::get};
use mongodb::bson::doc;
use serde::Serialize;

use crate::{AppError, adapters::http::app_state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(health))
        .route("/ready", get(readiness))
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

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
