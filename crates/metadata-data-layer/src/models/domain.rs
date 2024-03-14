use chrono::{DateTime, Utc};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Domain {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Domain {
    pub fn new(name: impl ToString) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            name: name.to_string().to_lowercase(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Domain(id={}, name={})", self.id, self.name)
    }
}
