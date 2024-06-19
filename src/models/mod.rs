pub mod shop;
pub mod key_token;

use mongodb::Database;

use crate::error::Result;

pub async fn create_all_collections(db: &Database) -> Result<()> {
    shop::schema(db).await?;
    key_token::schema(db).await?;
    Ok(())
}
