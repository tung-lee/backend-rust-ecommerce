use axum::{extract::Request, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::error::Result;

pub struct AccessController;

impl AccessController {
    pub async fn sign_up(req: Request) -> Result<impl IntoResponse> {
        println!("[P]::signUp::{:?}", req);
        Ok((
            StatusCode::OK,
            Json(json!({
                "code": "20001",
                "metadata": json!({
                    "userId": 1
                })
            })),
        ))
    }
}
