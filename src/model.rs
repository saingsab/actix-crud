use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TodoModel {
    pub id: Uuid,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub create_at: Option<chrono::DataTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DataTime<chrono::Utc>>
}