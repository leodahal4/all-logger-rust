use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub type DB = Pool<Postgres>;
use crate::Config;

pub async fn db_connection(config: &crate::Config) -> Result<DB, sqlx::Error> {
    {
        // migrate the db sql files
        // NOTE: THIS SHOULD BE ONLY IN THE DEVELOPMENT MODE,
        // so checking the env for environment
        if config.production == true {
            println!("PRODUCTION SERVER FOUND")
        } else {
            println!("DEVELOPMENT SERVER FOUND")
        }
    }
    db_pool(&config).await
}

async fn db_pool(config: &Config) -> Result<DB, sqlx::Error> {
    let connection_string = format!("postgres://{}:{}@{}//{}", config.db_user, config.db_pass, config.db_host, config.db_name);

    PgPoolOptions::new()
        .connect(&connection_string)
        .await
}
