use sqlx::{PgPool, postgres::PgQueryResult};

pub async fn save_chat(
    pool: &PgPool,
    user_query: &str,
    ai_response: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO chat_history (user_query, ai_response)
    VALUES ($1, $2)
    "#,
        user_query,
        ai_response
    )
    .execute(pool)
    .await
}
