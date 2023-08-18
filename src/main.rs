use axum::prelude::*;
use axum::response::IntoResponse;
use axum::AddExtensionLayer;
use tower::ServiceExt;

mod app;
mod handler;

#[tokio::main]
async fn main() {
    let app = app::App::new()
        .router(
            Router::new()
                .route("/sum", post(handler::handle_sum))
                .route("/multiply", post(handler::handle_multiply))
                .route("/subtract", post(handler::handle_subtract)),
        )
        .build();

    if let Err(err) = app.await {
        eprintln!("Server error: {}", err);
    }
}