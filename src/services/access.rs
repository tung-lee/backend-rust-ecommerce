use mongodb::{bson::doc, Database};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};

use crate::{
    constants::collection_name::SHOP_COLLECTION,
    error::{Error, Result},
    models::shop::{CreateShop, Shop, ShopRole, ShopStatus},
};

pub struct AccessService;

impl AccessService {
    pub async fn sign_up(db: &Database, payload: CreateShop) -> Result<()> {
        if let Some(_) = db
            .collection::<Shop>(SHOP_COLLECTION)
            .find_one(
                doc! {
                    "email": payload.email.clone()
                },
                None,
            )
            .await
            .map_err(|e| Error::Other(e.to_string()))?
        {
            return Err(Error::Other("Shop already registered!".to_string()));
        } else {
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = Pbkdf2
                .hash_password(payload.password.as_bytes(), &salt)
                .map_err(|e| Error::Other(e.to_string()))?
                .to_string();
            let shop = Shop {
                name: payload.name,
                email: payload.email,
                password: password_hash,
                status: ShopStatus::Active,
                verify: false,
                roles: vec![ShopRole::Shop],
            };
            db.collection::<Shop>(SHOP_COLLECTION)
                .insert_one(shop, None)
                .await
                .unwrap();
        }
        Ok(())
    }
}
