use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[doc = "Application status check"]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApplicationStatus {
    pub is_initialized: Option<bool>,
}

impl ApplicationStatus {
    pub async fn get_application_status(pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT EXISTS(SELECT ur.role
              FROM user_role ur
              WHERE ur.role = 'AUTHGATE_ADMIN')
           AS is_initialized
            "#
        )
        .fetch_one(pool)
        .await
    }
}
