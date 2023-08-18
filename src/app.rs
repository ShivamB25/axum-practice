use axum::{handler::post, Router};
use tower::ServiceBuilder;

use crate::handler::{handle_sum, handle_multiply, handle_subtract};

pub struct App {
    router: Router<Body>,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn router(self, router: Router<Body>) -> Self {
        Self {
            router: self.router.or(router),
        }
    }

    pub fn build(self) -> axum::Server<impl axum::Service<Request = axum::Body, Response = axum::Body, Error = std::convert::Infallible> + Clone + Send + Sync + 'static> {
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(self.router.into_make_service())
    }
}