use mongodb::{
    bson::{doc, Bson},
    options::{CreateCollectionOptions, ValidationAction, ValidationLevel},
    Database,
};
use serde::{Deserialize, Serialize};

use crate::{
    constants::collection_name::SHOP_COLLECTION,
    error::{Error, Result},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shop {
    pub name: String,
    pub email: String,
    pub password: String,
    pub status: ShopStatus,
    pub verify: bool,
    pub roles: Vec<ShopRole>,
}

#[derive(Debug, Serialize)]
pub struct ShopDTO {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShop {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShopStatus {
    Active,
    Inactive,
}

impl From<ShopStatus> for Bson {
    fn from(status: ShopStatus) -> Self {
        match status {
            ShopStatus::Active => Bson::String("Active".to_string()),
            ShopStatus::Inactive => Bson::String("Inactive".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShopRole {
    Shop,
    Write,
    Editor,
    Admin,
}

impl From<ShopRole> for Bson {
    fn from(role: ShopRole) -> Self {
        match role {
            ShopRole::Shop => Bson::String("Shop".to_string()),
            ShopRole::Write => Bson::String("Write".to_string()),
            ShopRole::Editor => Bson::String("Editor".to_string()),
            ShopRole::Admin => Bson::String("Admin".to_string()),
        }
    }
}

pub async fn schema(db: &Database) -> Result<()> {
    let validator = doc! {
        "$jsonSchema": doc! {
           "bsonType": "object",
           "title": "Shop Object Validation",
           "required": vec![ "password" ],
           "properties": doc! {
              "name": doc! {
                  "bsonType": "string",
                  "maxLength": 150
              },
              "email": doc! {
                  "bsonType": "string",
              },
              "password": doc! {
                  "bsonType": "string",
              },
              "status": doc! {
                  "enum": vec! [ShopStatus::Active, ShopStatus::Inactive],
              },
              "verify": doc! {
                  "bsonType": "bool",
                },
              "roles": doc! {
                  "bsonType": "array",
                  "items": {
                      "enum": vec! [
                          ShopRole::Shop,
                          ShopRole::Write,
                          ShopRole::Editor,
                          ShopRole::Admin,],
                    },
              },
           }
        }
    };

    let validation_opts = CreateCollectionOptions::builder()
        .validator(validator)
        .validation_action(Some(ValidationAction::Error))
        .validation_level(Some(ValidationLevel::Moderate))
        .build();

    db.create_collection(SHOP_COLLECTION, validation_opts)
        .await
        .map_err(|_| Error::Other("Create Collection Failed".to_string()))?;

    Ok(())
}
