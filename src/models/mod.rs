pub mod shop;

use mongodb::Database;

use crate::error::Result;

pub async fn create_all_collections(db: &Database) -> Result<()> {
    shop::schema(db).await?;
    Ok(())
}
