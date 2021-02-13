use super::domain::DomainRepository;
use super::model::ShadowverseAPIResponse;
use async_trait::async_trait;
use diesel::pg::PgConnection;
use reqwest;
use serde_json;
use std::error::Error;

pub struct ShadowverseRepository {
    db: PgConnection,
}

#[async_trait]
impl DomainRepository for ShadowverseRepository {
    fn new(conn: PgConnection) -> ShadowverseRepository {
        ShadowverseRepository { db: conn }
    }
    async fn get_new_cards() -> Result<ShadowverseAPIResponse, Box<dyn Error>> {
        let body = reqwest::get("https://shadowverse-portal.com/api/v1/cards?format=json&lang=en")
            .await?
            .text()
            .await?;
        let result: ShadowverseAPIResponse = serde_json::from_str(body.as_str())?;
        Ok(result)
    }

    async fn update_db() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
