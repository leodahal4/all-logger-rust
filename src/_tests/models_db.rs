use crate::models::db_connection;
use crate::env_reader;

#[tokio::test]
async fn models_db_db_connection() -> Result<(), Box<dyn std::error::Error>> {
    let config = env_reader::read_config();
    let db = db_connection(&config).await?;
    
    println!("{:?}", db);
    Ok(())
}

#[tokio::test]
async fn models_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    let config = env_reader::read_config();
    let db = db_connection(&config).await?;
    
    let rows = sqlx::query("select * from log").fetch_all(&db).await?;
    assert_eq!(6, rows.len(), "initial rows matched");

    Ok(())
}
