use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::db::init_mongodb::connect;
use crate::error::Result;
use crate::helpers::check_db_connection::check_overload;
use crate::models::create_all_collections;
use crate::routes::create_router;

pub async fn get_app() -> Result<Router> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // init db
    let client = connect(None).await?;
    check_overload(&client).await;
    let db = client.database("backend-ecomerce");
    let _ = create_all_collections(&db).await?;

    // init routes
    let app = create_router().layer(
        ServiceBuilder::new()
            .layer(CompressionLayer::new())
            .layer(TraceLayer::new_for_http())
            .layer(Extension(db)),
    );
    // init middlewares
    Ok(app)
}
