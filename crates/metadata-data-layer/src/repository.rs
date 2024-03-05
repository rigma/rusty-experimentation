use sqlx::{pool::PoolConnection, Database};

pub trait Repository {
    fn from_connection<DB>(conn: PoolConnection<DB>) -> Self
    where
        DB: Database;
}
