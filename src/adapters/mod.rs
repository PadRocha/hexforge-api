//! Adaptadores de entrada y salida.
//!
//! # Arquitectura
//!
//! Los adaptadores traducen protocolos o tecnologías concretas hacia contratos
//! de aplicación. En esta plantilla existen adaptadores HTTP con Axum y un
//! espacio reservado para persistencia con MongoDB.

/// Adaptador de entrada HTTP construido con Axum.
pub mod http;
/// Adaptadores de salida para persistencia.
pub mod persistence;
