//! Conversión de errores de aplicación a respuestas HTTP.
//!
//! Este módulo mantiene estable el contrato de error expuesto por el adaptador
//! HTTP sin mezclar detalles de serialización dentro de la capa de aplicación.

use crate::AppError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    code: &'static str,
    message: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the error before it gets converted into a status response.
        tracing::error!(error = ?self, "Request failed");

        let (status, code, message) = match self {
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "bad_request", "Bad request"),
            AppError::ValidationFailed => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_failed",
                "Validation failed",
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not_found", "Resource not found"),
            AppError::Conflict => (StatusCode::CONFLICT, "conflict", "Resource conflict"),
            AppError::Unauthenticated => (
                StatusCode::UNAUTHORIZED,
                "unauthenticated",
                "Authentication is required",
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "forbidden",
                "You are not allowed to perform this action",
            ),
            AppError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limited",
                "Too many requests",
            ),
            AppError::NotImplemented => (
                StatusCode::NOT_IMPLEMENTED,
                "not_implemented",
                "Feature not implemented",
            ),
            AppError::BadGateway => (
                StatusCode::BAD_GATEWAY,
                "bad_gateway",
                "Upstream dependency failed",
            ),
            AppError::GatewayTimeout => (
                StatusCode::GATEWAY_TIMEOUT,
                "gateway_timeout",
                "Upstream dependency timed out",
            ),
            AppError::PersistenceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "persistence_unavailable",
                "Persistence service unavailable",
            ),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal",
                "Internal server error",
            ),
        };

        (
            status,
            Json(ErrorResponse {
                status: status.as_u16(),
                code,
                message,
            }),
        )
            .into_response()
    }
}
