use mongodb::{bson::doc, Database};
use openssl::{pkey::PKey, rsa};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};

use crate::{
    auth::auth_utils::create_token_pair,
    constants::collection_name::SHOP_COLLECTION,
    error::{Error, Result},
    models::{
        key_token::CreateKeyToken,
        shop::{CreateShop, Shop, ShopRole, ShopStatus},
    },
    utils::jwt::UserData,
};

use super::key_token::KeyTokenService;

pub struct AccessService;

impl AccessService {
    pub async fn sign_up(db: &Database, payload: CreateShop) -> Result<(Shop, (String, String))> {
        // Check if the shop already exists
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
            // Hash the password
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = Pbkdf2
                .hash_password(payload.password.as_bytes(), &salt)
                .map_err(|e| Error::Other(e.to_string()))?
                .to_string();

            // Insert the shop
            let shop = Shop {
                name: payload.name,
                email: payload.email.clone(),
                password: password_hash,
                status: ShopStatus::Inactive.into(),
                verify: false,
                roles: vec![ShopRole::Shop.into()],
            };

            let insert_result = db
                .collection::<Shop>(SHOP_COLLECTION)
                .insert_one(shop.clone(), None)
                .await
                .map_err(|e| Error::Other(e.to_string()))?;

            // Generate RSA keypair
            let keypair = rsa::Rsa::generate(2048).map_err(|e| Error::Other(e.to_string()))?;

            let public_key = keypair
                .public_key_to_pem()
                .map_err(|e| Error::Other(e.to_string()))?;

            let private_key = keypair
                .private_key_to_pem()
                .map_err(|e| Error::Other(e.to_string()))?;

            // Save public key to database
            let user_id = insert_result
                .inserted_id
                .as_object_id()
                .ok_or_else(|| Error::Other("Failed to get inserted id".to_string()))?;

            let data = CreateKeyToken {
                user_id,
                public_key: public_key.clone(),
            };

            let public_key_str = KeyTokenService::create_key_token(db, data).await?;

            let public_key_obj = PKey::public_key_from_pem(public_key_str.as_bytes())
                .map_err(|e| Error::Other(e.to_string()))?;

            // create access token, refresh token
            let token_pair = create_token_pair(
                UserData {
                    user_id,
                    email: payload.email,
                },
                public_key_obj,
                private_key,
            )?;
            return Ok((shop, token_pair));
        }
    }
}
