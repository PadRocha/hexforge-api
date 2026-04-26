//! Plantilla base para APIs HTTP en Rust con Axum, MongoDB y separación por
//! capas.
//!
//! # Arquitectura
//!
//! El crate está organizado como una variante práctica de clean/hexagonal:
//!
//! - [`domain`] contiene reglas y tipos puros del negocio.
//! - [`application`] coordina casos de uso y define contratos de entrada/salida.
//! - [`adapters`] integra tecnologías externas como HTTP y persistencia.
//! - [`infra`] ensambla configuración, base de datos, tracing y arranque.
//!
//! La intención es que el dominio y la aplicación puedan crecer sin depender
//! directamente de Axum ni del driver de MongoDB.

/// Adaptadores que conectan la aplicación con tecnologías externas.
pub mod adapters;
/// Casos de uso, puertos y errores propios de la capa de aplicación.
pub mod application;
/// Modelo y reglas de dominio independientes de infraestructura.
pub mod domain;
/// Configuración y ensamblaje técnico de la API.
pub mod infra;

/// Errores de aplicación compartidos por handlers y casos de uso.
pub use application::app_error::{AppError, AppResult};
