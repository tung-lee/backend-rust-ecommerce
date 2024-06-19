use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<T> {
    pub data: T,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub user_id: ObjectId,
    pub email: String,
}

pub fn sign(payload: UserData, exp: usize, rsa_key: Vec<u8>) -> Result<String> {
    let claims = Claims::<UserData> { data: payload, exp };

    let header = Header {
        alg: jsonwebtoken::Algorithm::RS256,
        ..Default::default()
    };

    let encoding_key =
        EncodingKey::from_rsa_pem(rsa_key.as_ref()).map_err(|e| Error::Other(e.to_string()))?;

    let token = encode(&header, &claims, &encoding_key).map_err(|e| Error::Other(e.to_string()))?;

    Ok(token)
}

pub fn verify(token: String, rsa_key: Vec<u8>) -> Result<Claims<UserData>> {
    let decoding_key =
        DecodingKey::from_rsa_pem(rsa_key.as_ref()).map_err(|e| Error::Other(e.to_string()))?;

    let validation = Validation::new(Algorithm::RS256);

    let claims = decode::<Claims<UserData>>(&token, &decoding_key, &validation)
        .map_err(|e| Error::Other(format!("Decoding failed: {}", e.to_string())))?
        .claims;
    Ok(claims)
}
