use anyhow::Result;
use loredb::storage::{Database, Entity};

use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to LoreDB!");
    println!("---  Structured memory for intelligent agents--- ");

    test_basic_setup().await?;

    //Create database

    println!("Creating SQLite DB in path: {:?}", std::env::current_dir()?);
    let db = Database::new("sqlite://loredb.db").await?;
    println!("Database connection established");

    //initialize schema
    db.initialize().await?;
    println!("Database schema initialized!");

    //create tets entity
    let entity = Entity::new(
        "ent_john_001".to_string(),
        "person".to_string(),
        "John Doe".to_string(),
        json!({
            "age": 30,
            "occupation": "Software Engineer"
        }),
        json!({
            "source": "test_input",
            "confidence": 0.95
        }),
    );

    println!("\n📝 Inserting entity: {}", entity.name);
    println!("   ID: {}", entity.id);
    println!("   Type: {}", entity.entity_type);

    db.insert_entity(entity).await?;

    println!("✅ Entity inserted successfully!");

    println!("\n🎉 LoreDB is working!");

    Ok(())
}

async fn test_basic_setup() -> Result<()> {
    println!("Rust setup working!");
    println!("Tokio async runtime working!");

    Ok(())
}
