use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use mongodb::Database;
use serde_json::json;

use crate::{
    error::Result,
    models::shop::{CreateShop, ShopDTO},
    services::access::AccessService,
};

pub struct AccessController;

impl AccessController {
    pub async fn sign_up(
        Extension(db): Extension<Database>,
        Json(data): Json<CreateShop>,
    ) -> Result<impl IntoResponse> {
        println!("[P]::signUp::{:?}", data);
        let (shop, token_pair) = AccessService::sign_up(&db, data).await?;
        let shop_dto = ShopDTO {
            name: shop.name,
            email: shop.email,
        };
        Ok((
            StatusCode::OK,
            Json(json!({
                "code": "201",
                "metadata": json!({
                    "shop": shop_dto,
                    "token_pair": token_pair
                })
            })),
        ))
    }
}
