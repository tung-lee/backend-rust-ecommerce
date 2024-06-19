use std::str::from_utf8;

use mongodb::bson::doc;
use mongodb::Database;

use crate::constants::collection_name::KEY_TOKEN_COLLECTION;
use crate::models::key_token::KeyToken;
use crate::{error::Error, models::key_token::CreateKeyToken};

use crate::error::Result;

pub struct KeyTokenService;

impl KeyTokenService {
    pub async fn create_key_token(db: &Database, payload: CreateKeyToken) -> Result<String> {
        let public_key_str = from_utf8(payload.public_key.as_slice())
            .map_err(|e| Error::Other(e.to_string()))?
            .to_string();

        let insert_result = db
            .collection(KEY_TOKEN_COLLECTION)
            .insert_one(
                KeyToken {
                    user_id: payload.user_id,
                    public_key: public_key_str.clone(),
                    refresh_token: Vec::new(),
                },
                None,
            )
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        let insert_data = db
            .collection::<KeyToken>(KEY_TOKEN_COLLECTION)
            .find_one(
                doc! {
                    "_id": insert_result.inserted_id.as_object_id().unwrap()
                },
                None,
            )
            .await
            .map_err(|e| Error::Other(e.to_string()))?
            .ok_or_else(|| Error::Other("Key token not found".to_string()))?;

        Ok(insert_data.public_key)
    }
}
