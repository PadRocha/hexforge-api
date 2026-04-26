use serde::Serialize;

/// Respuesta JSON para endpoints que devuelven un recurso o colección.
#[derive(Debug, Clone, Serialize)]
pub struct DataResponse<T> {
    /// Carga útil del endpoint.
    pub data: T,
}

/// Respuesta JSON mínima para endpoints que solo comunican un resultado textual.
#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    /// Mensaje estable y apto para clientes HTTP.
    pub message: &'static str,
}
