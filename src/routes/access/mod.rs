use axum::{routing::post, Router};

use crate::controllers::access::AccessController;

pub fn create_router() -> Router {
    Router::new().route("/shop/sign-up", post(AccessController::sign_up))
}
