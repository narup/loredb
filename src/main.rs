use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to LoreDB!");
    println!("---  Structured memory for intelligent agents--- ");


    test_basic_setup().await?;
    Ok(())
}

async fn test_basic_setup() -> Result<()> {
    println!("Rust setup working!");
    println!("Tokio async runtime working!");

    Ok(())
}
