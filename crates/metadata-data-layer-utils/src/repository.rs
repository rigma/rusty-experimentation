use std::sync::Arc;

/// A trait to tag a structure as a repository that can be
/// extracted thanks to [Repository](metadata_data_layer_utils::extract::Repository).
pub trait Repository {
    type DB;

    fn from_ref(pool: Arc<sqlx::Pool<Self::DB>>) -> Self
    where
        <Self as Repository>::DB: sqlx::Database;
}
