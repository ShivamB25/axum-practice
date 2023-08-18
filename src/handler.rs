use axum::{extract::Json, response::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CalculationRequest {
    num1: i32,
    num2: i32,
}

#[derive(Debug, Serialize)]
pub struct CalculationResponse {
    result: i32,
}

pub async fn handle_sum(Json(request): Json<CalculationRequest>) -> Json<CalculationResponse> {
    let result = request.num1 + request.num2;
    Json(CalculationResponse { result })
}

pub async fn handle_multiply(Json(request): Json<CalculationRequest>) -> Json<CalculationResponse> {
    let result = request.num1 * request.num2;
    Json(CalculationResponse { result })
}

pub async fn handle_subtract(Json(request): Json<CalculationRequest>) -> Json<CalculationResponse> {
    let result = request.num1 - request.num2;
    Json(CalculationResponse { result })
}