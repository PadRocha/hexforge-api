//! Adaptador HTTP de la API.
//!
//! # Responsabilidad
//!
//! Contiene el estado compartido de Axum, rutas, tipos de respuesta y la
//! traducción de errores de aplicación a respuestas HTTP. Los handlers deben
//! delegar reglas de negocio a la capa de aplicación cuando existan casos de
//! uso concretos.

/// Traducción de errores de aplicación al protocolo HTTP.
pub mod app_error_impl;
/// Estado compartido inyectado en handlers y extractores.
pub mod app_state;
/// Envoltorios JSON reutilizables para respuestas HTTP.
pub mod responses;
/// Definición y composición de rutas HTTP.
pub mod routes;
