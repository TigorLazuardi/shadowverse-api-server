use crate::services::http::card::model::Card;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShadowverseAPIResponse {
    pub data_headers: DataHeaders,
    pub data: Data,
}

#[derive(Deserialize)]
pub struct DataHeaders {
    pub udid: bool,
    pub viewer_id: u32,
    pub sid: String,
    pub servertime: u64,
    pub result_code: u8,
}

#[derive(Deserialize)]
pub struct Data {
    pub cards: Vec<Card>,
    pub errors: Vec<String>,
}
