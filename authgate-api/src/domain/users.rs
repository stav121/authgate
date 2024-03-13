use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[doc = "Entity that maps to @users table"]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id:         Uuid,
    pub username:   Option<String>,
    pub email:      Option<String>,
    pub password:   String,
    pub locked:     bool,
    pub last_login: Option<chrono::DateTime<Utc>>,
    pub created_at: chrono::DateTime<Utc>,
}

impl User {
    /// Find a user by @users.id.
    ///
    /// # Args
    ///
    /// * id - The UUID of the user
    /// * pool - The Postgres connection pool
    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT u.*
            FROM users u
            WHERE u.id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }
}
