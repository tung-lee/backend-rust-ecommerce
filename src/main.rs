mod app;
mod db;
mod error;
mod helpers;

use db::init_mongodb::connect;
use error::Result;
use helpers::check_db_connection::check_overload;

use app::get_app;

#[tokio::main]
async fn main() -> Result<()> {
    let app = get_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let client = connect(None).await?;
    check_overload(&client).await;

    println!("Server running on: http://localhost:3000");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
