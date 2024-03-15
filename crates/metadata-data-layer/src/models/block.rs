use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Serialize)]
#[serde(tag = "type")]
pub enum Parent {
    Block(Uuid),
    Domain(Uuid),
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Block {
    pub id: Uuid,
    pub parent: Parent,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Block {
    #[inline]
    pub fn builder() -> BlockBuilder {
        BlockBuilder::default()
    }
}

impl FromRow<'_, PgRow> for Block {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let name = row.try_get("name")?;
        let created_at = row.try_get("created_at")?;
        let updated_at = row.try_get("updated_at")?;

        let domain_id: Option<Uuid> = row.try_get("domain_id")?;
        let block_id: Option<Uuid> = row.try_get("block_id")?;

        let parent = match (domain_id, block_id) {
            (Some(uuid), None) => Parent::Domain(uuid),
            (None, Some(uuid)) => Parent::Block(uuid),
            // TODO(rigma): assert that this portion is unreachable due to SQL constraints
            _ => unreachable!(),
        };

        Ok(Self {
            id,
            parent,
            name,
            created_at,
            updated_at,
        })
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block(id={}, name={})", &self.id, &self.name)
    }
}

#[derive(Clone, Debug, Default)]
pub struct BlockBuilder {
    parent: Option<Parent>,
    name: Option<String>,
}

impl BlockBuilder {
    pub fn block(mut self, uuid: Uuid) -> Self {
        self.parent = Some(Parent::Block(uuid));
        self
    }

    pub fn domain(mut self, uuid: Uuid) -> Self {
        self.parent = Some(Parent::Domain(uuid));
        self
    }

    pub fn name(mut self, name: &impl ToString) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn finalize(self) -> Result<Block, ()> {
        // TODO(rigma): add an error type to indicate what is missing
        let name = self.name.ok_or(())?;
        let parent = self.parent.ok_or(())?;
        let now = Utc::now();

        Ok(Block {
            id: Uuid::now_v7(),
            parent,
            name,
            created_at: now,
            updated_at: now,
        })
    }
}
