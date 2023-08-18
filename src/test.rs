use axum::{body::Body, http::StatusCode, response::IntoResponse, Router};
use serde_json::json;
use tower::ServiceExt;

use crate::app::App;

#[tokio::test]
async fn test_sum_endpoint() {
    let app = App::new();
    let router = Router::new().route("/sum", axum::post(handler::handle_sum));
    let app = app.router(router);

    let request = axum::testing::Request::post("/sum")
        .header("Content-Type", "application/json")
        .body(json!({"num1": 5, "num2": 3}).to_string())
        .unwrap();

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to send request")
        .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body()).await.unwrap();
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(result, json!({"result": 8}));
}

#[tokio::test]
async fn test_multiply_endpoint() {
    let app = App::new();
    let router = Router::new().route("/multiply", axum::post(handler::handle_multiply));
    let app = app.router(router);

    let request = axum::testing::Request::post("/multiply")
        .header("Content-Type", "application/json")
        .body(json!({"num1": 5, "num2": 3}).to_string())
        .unwrap();

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to send request")
        .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body()).await.unwrap();
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(result, json!({"result": 15}));
}

#[tokio::test]
async fn test_subtract_endpoint() {
    let app = App::new();
    let router = Router::new().route("/subtract", axum::post(handler::handle_subtract));
    let app = app.router(router);

    let request = axum::testing::Request::post("/subtract")
        .header("Content-Type", "application/json")
        .body(json!({"num1": 5, "num2": 3}).to_string())
        .unwrap();

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to send request")
        .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body()).await.unwrap();
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(result, json!({"result": 2}));
}

#[tokio::test]
async fn test_invalid_input() {
    let app = App::new();
    let router = Router::new().route("/sum", axum::post(handler::handle_sum));
    let app = app.router(router);

    let request = axum::testing::Request::post("/sum")
        .header("Content-Type", "application/json")
        .body(json!({"num1": "invalid", "num2": 3}).to_string())
        .unwrap();

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to send request")
        .into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}