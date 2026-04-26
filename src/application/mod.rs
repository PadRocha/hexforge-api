//! Capa de aplicación.
//!
//! # Responsabilidad
//!
//! Aquí viven los casos de uso, servicios de aplicación, DTO internos y
//! puertos que expresan lo que la API necesita del exterior. Esta capa debe
//! coordinar reglas de negocio sin conocer detalles de Axum, MongoDB u otros
//! adaptadores concretos.

/// Error común para expresar fallos de aplicación hacia adaptadores externos.
pub mod app_error;
