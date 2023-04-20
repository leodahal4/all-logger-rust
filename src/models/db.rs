use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::Config;
use std::{fs, path::PathBuf};

const SQL_DIR: &str = "sql/";
const SQL_INITIAL: &str = "sql/00-recreate-db.sql";

pub type DB = Pool<Postgres>;

/// This must only be runned for the first time the db is initialized
pub async fn init_db(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_sql: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    
    all_sql.sort();
    let conn = db_pool(&config).await?;

    for sql in all_sql {
        if let Some(sql) = sql.to_str() {
            if sql.ends_with(".sql") && sql == SQL_INITIAL {continue;}
            first_init(&conn, &sql).await?;
        }
    }
    Ok(())
}

async fn first_init(db: &DB, file: &str) -> Result<(), sqlx::Error> {
    let statements = fs::read_to_string(file).map_err(|e| {
        println!("Error the sql file {}.\nError: {}", file, e);
        e
    })?;
    let sqls: Vec<&str> = statements.split(";").collect();

    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(e) => println!("Error exectuing the sql statement on file {}.\nError:{}", sql, e)
        }
    }
    Ok(())
}

pub async fn db_connection(config: &crate::Config) -> Result<DB, sqlx::Error> {
    {
        // migrate the db sql files
        // NOTE: THIS SHOULD BE ONLY IN THE DEVELOPMENT MODE,
        // so checking the env for environment
        if config.is_prod_env(){
            println!("PRODUCTION SERVER FOUND")
        } else {
            println!("DEVELOPMENT SERVER FOUND")
        }
    }

    db_pool(&config).await
}

async fn db_pool(config: &Config) -> Result<DB, sqlx::Error> {
    let connection_string = format!("postgres://{}:{}@{}/{}", config.db_user, config.db_pass, config.db_host, config.db_name);

    PgPoolOptions::new()
        .connect(&connection_string)
        .await
}

#[cfg(test)]
#[path = "../_tests/models_db.rs"]
mod tests;
