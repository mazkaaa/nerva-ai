use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct ChatMessage {
    pub id: i32,
    pub user_query: String,
    pub ai_response: String,
    pub timestamp: Option<DateTime<Utc>>,
}
