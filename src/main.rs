mod app;
mod constants;
mod db;
mod error;
mod helpers;
mod models;
mod routes;
mod services;
mod controllers;
mod auth;
mod utils;

use error::Result;

use app::get_app;

#[tokio::main]
async fn main() -> Result<()> {
    let app = get_app().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on: http://localhost:3000");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
