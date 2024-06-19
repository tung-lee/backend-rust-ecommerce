use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{CreateCollectionOptions, ValidationAction, ValidationLevel},
    Database,
};
use serde::{Deserialize, Serialize};

use crate::{
    constants::collection_name::KEY_TOKEN_COLLECTION,
    error::{Error, Result},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyToken {
    pub user_id: ObjectId, // shop
    pub public_key: String,
    pub refresh_token: Vec<String>,
}

pub struct CreateKeyToken {
    pub user_id: ObjectId,
    pub public_key: Vec<u8>,
}

pub async fn schema(db: &Database) -> Result<()> {
    let validator = doc! {
        "$jsonSchema": doc! {
           "bsonType": "object",
           "title": "KeyToken Object Validation",
        //    "required": vec! [ "user_id, public_key" ],
           "properties": doc! {
              "user_id": doc! {
                  "bsonType": "objectId",
              },
              "public_key": doc! {
                  "bsonType": "string",
              },
              "refresh_token": doc! {
                  "bsonType": "array",
                  "items": {
                      "bsonType": "string",
                  }
              },
           }
        }
    };

    let validation_opts = CreateCollectionOptions::builder()
        .validator(validator)
        .validation_action(Some(ValidationAction::Error))
        .validation_level(Some(ValidationLevel::Moderate))
        .build();

    db.create_collection(KEY_TOKEN_COLLECTION, validation_opts)
        .await
        .map_err(|_| Error::Other("Create Collection Failed".to_string()))?;

    Ok(())
}
