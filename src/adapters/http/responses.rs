use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DataResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub message: &'static str,
}
