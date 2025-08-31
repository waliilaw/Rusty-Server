use axum::{Json , response::IntoResponse};
use crate::types::ApiResponse;

pub async fn health() -> impl IntoResponse{
    let response = ApiResponse{
        success : true,
        data: Some("Service is healthy"),
        error : None
    };
    Json(response)
}
