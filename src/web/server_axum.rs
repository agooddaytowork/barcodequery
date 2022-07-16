use std::net::SocketAddr;

use axum::http::Uri;
use axum::{http::StatusCode, routing::get, Extension, Router};
use hyper;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::web::handlers;
use crate::web::state::SharedState;

pub struct WebSocketServer {}

impl WebSocketServer {
    pub async fn run_until_stopped(shared_state: SharedState) -> hyper::Result<()> {
        println!("running in websocket");
        let app = Router::new()
            .route("/ws", get(handlers::ws_handler))
            .route("/health_check", get(handlers::health_check_handler))
            .layer(
                ServiceBuilder::new()
                    .layer(
                        // logging so we can see whats going on
                        TraceLayer::new_for_http()
                            .make_span_with(DefaultMakeSpan::default().include_headers(true)),
                    )
                    .layer(Extension(shared_state)),
            );

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::info!("listening on {}", addr);
        let s = axum::Server::bind(&addr).serve(app.into_make_service());

        s.await
    }
}

async fn fallback_not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
