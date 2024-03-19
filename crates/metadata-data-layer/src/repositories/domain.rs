use crate::models::Domain;
use metadata_data_layer_utils::Repository;
use sqlx::{self, postgres::Postgres, Pool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct DomainRepository {
    pool: Arc<Pool<Postgres>>,
}

impl DomainRepository {
    pub async fn get_domain(&self, domain_id: &Uuid) -> Result<Option<Domain>, sqlx::Error> {
        sqlx::query_as::<_, Domain>(
            r#"
            SELECT
                domains.id,
                domains.name,
                domains.created_at,
                domains.updated_at
            FROM domains
            WHERE domains.id = $1
            "#,
        )
        .bind(domain_id)
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn get_domain_by_name(
        &self,
        domain_name: &str,
    ) -> Result<Option<Domain>, sqlx::Error> {
        sqlx::query_as::<_, Domain>(
            r#"
            SELECT
                domains.id,
                domains.name,
                domains.created_at,
                domains.updated_at
            FROM domains
            WHERE domains.name = $1
            "#,
        )
        .bind(domain_name)
        .fetch_optional(self.pool.as_ref())
        .await
    }
}

impl Repository for DomainRepository {
    type DB = Postgres;

    fn from_pool(pool: Arc<Pool<Self::DB>>) -> Self {
        Self { pool }
    }
}
