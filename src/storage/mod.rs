use anyhow::{Ok, Result};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

mod types;
pub use types::*;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    //create a new database connection
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn initialize(&self) -> Result<()> {
        let schema = include_str!("schema.sql");
        sqlx::raw_sql(schema).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn insert_entity(&self, entity: Entity) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO entities(id, type, name, properties, first_seen, last_updated, metadata)
        values(?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        )
        .bind(entity.id)
        .bind(entity.entity_type)
        .bind(entity.name)
        .bind(entity.properties.to_string())
        .bind(entity.first_seen)
        .bind(entity.last_updated)
        .bind(entity.metadata.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
