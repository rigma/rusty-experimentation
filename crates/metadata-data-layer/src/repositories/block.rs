use crate::{internal::repository::Repository, models::Block};
use sqlx::{self, postgres::Postgres, Pool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct BlockRepository {
    pool: Arc<Pool<Postgres>>,
}

impl BlockRepository {
    pub async fn get_block(&self, block_id: &Uuid) -> Result<Option<Block>, sqlx::Error> {
        sqlx::query_as::<_, Block>(
            r#"
            SELECT
                blocks.id,
                blocks.domain_id,
                blocks.block_id,
                blocks.name,
                blocks.created_at,
                blocks.updated_at
            FROM blocks
            WHERE blocks.id = $1
            "#,
        )
        .bind(block_id)
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn get_block_by_name(&self, block_name: &str) -> Result<Option<Block>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                blocks.id,
                blocks.domain_id,
                blocks.block_id,
                blocks.name,
                blocks.created_at,
                blocks.updated_at
            FROM blocks
            WHERE blocks.name = $1
            "#,
        )
        .bind(block_name)
        .fetch_optional(self.pool.as_ref())
        .await
    }
}

impl Repository for BlockRepository {
    type DB = Postgres;

    fn from_pool(pool: Arc<sqlx::Pool<Self::DB>>) -> Self {
        Self { pool }
    }
}
