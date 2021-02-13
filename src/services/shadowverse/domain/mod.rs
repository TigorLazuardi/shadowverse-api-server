use super::model::ShadowverseAPIResponse;
use async_trait::async_trait;
use diesel::pg::PgConnection;
use std::error::Error;

#[async_trait]
pub trait DomainRepository {
    fn new(conn: PgConnection) -> Self;

    /// Get new cards from Shadowverse Portal. Warning 4MB in response size
    async fn get_new_cards() -> Result<ShadowverseAPIResponse, Box<dyn Error>>;

    // updates the database
    async fn update_db() -> Result<(), Box<dyn Error>>;
}
