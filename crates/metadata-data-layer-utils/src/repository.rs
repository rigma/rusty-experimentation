use std::sync::Arc;

pub trait Repository {
    type DB;

    fn from_pool(pool: Arc<sqlx::Pool<Self::DB>>) -> Self
    where
        <Self as Repository>::DB: sqlx::Database;
}
