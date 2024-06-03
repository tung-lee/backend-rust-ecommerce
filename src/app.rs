use axum::{http::StatusCode, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

pub fn get_app() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // init db

    // init routes

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                let str_test_compress = "Welcome Rust".to_string();
                (StatusCode::OK, str_test_compress.repeat(100))
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(TraceLayer::new_for_http()),
        );
    // init middlewares
    app
}
