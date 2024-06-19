use openssl::pkey::{PKey, Public};

use crate::{
    error::Result,
    utils::jwt::{self, UserData},
};

// access token, refresh token
pub fn create_token_pair(
    payload: UserData,
    public_key: PKey<Public>,
    private_key: Vec<u8>,
) -> Result<(String, String)> {
    let exp_access_token = (chrono::Utc::now() + chrono::Duration::days(2)).timestamp() as usize;
    let access_token = jwt::sign(payload.clone(), exp_access_token, private_key.clone())?;

    let exp_refresh_token = (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize;
    let refresh_token = jwt::sign(payload, exp_refresh_token, private_key.clone())?;

    let rsa_key = public_key
        .public_key_to_pem()
        .map_err(|e| crate::error::Error::Other(e.to_string()))?;
    let claims = jwt::verify(access_token.clone(), rsa_key)?;

    Ok((access_token, refresh_token))
}
