//! Infraestructura y composición técnica.
//!
//! # Responsabilidad
//!
//! Esta capa conecta configuración, logging, MongoDB y creación del router raíz.
//! Debe contener detalles de arranque y dependencias externas que no pertenecen
//! al dominio ni a la capa de aplicación.

/// Construcción del router raíz y middlewares globales.
pub mod app;
/// Lectura y validación de configuración desde variables de entorno.
pub mod config;
/// Inicialización de MongoDB.
pub mod db;
/// Rutinas de arranque compartidas por el binario.
pub mod setup;
/// Errores que pueden abortar el arranque de la API.
pub mod startup_error;
