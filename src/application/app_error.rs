use thiserror::Error;

/// Error común de la capa de aplicación.
///
/// Representa fallos que pueden ser producidos por casos de uso o handlers y
/// que el adaptador HTTP traduce a respuestas estables para los clientes.
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppError {
    /// La solicitud no cumple el contrato mínimo esperado por el endpoint.
    #[error("Bad request")]
    BadRequest,

    /// Los datos recibidos son sintácticamente válidos, pero fallan reglas de
    /// validación de aplicación o dominio.
    #[error("Validation failed")]
    ValidationFailed,

    /// El recurso solicitado no existe o no es visible para el actor actual.
    #[error("Resource not found")]
    NotFound,

    /// La operación no puede completarse porque ya existe un recurso
    /// equivalente o se viola una restricción de unicidad.
    #[error("Resource already exists")]
    Conflict,

    /// La operación requiere autenticación y no se recibió una identidad válida.
    #[error("Unauthenticated")]
    Unauthenticated,

    /// El actor autenticado no tiene permisos para ejecutar la operación.
    #[error("Forbidden")]
    Forbidden,

    /// La solicitud fue rechazada por límites de uso o protección de tráfico.
    #[error("Too many requests")]
    RateLimited,

    /// El endpoint o caso de uso todavía no tiene implementación productiva.
    #[error("Not implemented")]
    NotImplemented,

    /// Una dependencia externa respondió con un fallo no recuperable.
    #[error("Bad gateway")]
    BadGateway,

    /// Una dependencia externa no respondió dentro del tiempo esperado.
    #[error("Gateway timeout")]
    GatewayTimeout,

    /// La capa de persistencia no está disponible para completar la operación.
    #[error("Persistence unavailable")]
    PersistenceUnavailable,

    /// Fallo inesperado que no debe exponer detalles internos al cliente.
    #[error("Internal server error")]
    Internal,
}

/// Alias para resultados producidos por la capa de aplicación.
pub type AppResult<T> = Result<T, AppError>;
