use mongodb::{
    bson::doc,
    options::{CreateCollectionOptions, ValidationAction, ValidationLevel},
    Database,
};
use serde::{Deserialize, Serialize};

use crate::{
    constants::collection_name::SHOP_COLLECTION,
    error::{Error, Result},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    pub name: String,
    pub email: String,
    pub password: String,
    pub status: ShopStatus,
    pub verify: bool,
    pub roles: Vec<ShopRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShop {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShopStatus {
    Active,
    Inactive,
}

impl ShopStatus {
    fn to_string(&self) -> String {
        match self {
            ShopStatus::Active => "active".to_string(),
            ShopStatus::Inactive => "inactive".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShopRole {
    Shop,
    Write,
    Editor,
    Admin,
}

impl ShopRole {
    fn to_string(&self) -> String {
        match self {
            ShopRole::Shop => "SHOP".to_string(),
            ShopRole::Write => "WRITE".to_string(),
            ShopRole::Editor => "EDITOR".to_string(),
            ShopRole::Admin => "ADMIN".to_string(),
        }
    }
}

pub async fn schema(db: &Database) -> Result<()> {
    let validator = doc! {
        "$jsonSchema": doc! {
           "bsonType": "object",
           "title": "Shop Object Validation",
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
                "verify": doc! {
                    "bsonType": "bool",
                },
                "status": doc! {
                    "enum": vec! [ShopStatus::Active.to_string(), ShopStatus::Inactive.to_string()],
              },
              "roles": doc! {
                    "bsonType": "array",
                    "items": {
                        "enum": vec! [
                            ShopRole::Shop.to_string(),
                            ShopRole::Write.to_string(),
                            ShopRole::Editor.to_string(),
                            ShopRole::Admin.to_string(),],
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
