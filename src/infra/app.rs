use axum::{
    Router, http,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

use crate::{adapters, adapters::http::app_state::AppState, infra::startup_error::StartupError};

pub fn create_app(app_state: AppState) -> Result<Router, StartupError> {
    let allowed_origin = app_state
        .config
        .cors_allowed_origin
        .parse::<http::HeaderValue>()
        .map_err(|source| StartupError::InvalidCorsOrigin { source })?;

    let cors = CorsLayer::new()
        .allow_origin(allowed_origin)
        .allow_methods([
            http::Method::DELETE,
            http::Method::GET,
            http::Method::OPTIONS,
            http::Method::PATCH,
            http::Method::POST,
            http::Method::PUT,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    Ok(Router::new()
        .nest("/api", adapters::http::routes::router())
        .with_state(app_state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                let request_id = Uuid::new_v4();
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                    request_id = %request_id
                )
            }),
        ))
}
