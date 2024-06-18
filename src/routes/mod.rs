use axum::Router;

pub mod access;
pub mod shop;

pub fn create_router() -> Router {
    Router::new().nest("/v1/api", Router::new().merge(access::create_router()))
}

// Router::new().route(
//     "/",
//     get(|| async {
//         let str_test_compress = "Welcome Rust".to_string();
//         (StatusCode::OK, str_test_compress.repeat(100))
//     }),
// )
