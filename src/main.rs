mod models;
mod env_reader;

use models::{init_db, db_connection};
use dotenv::dotenv;
use env_reader::Config;

/// The main function is the entry point of the Rust program. 
/// It loads the environment variables from the .env file, reads the configuration values, 
/// establishes a database connection using the supplied configuration and prints the connection details.
#[tokio::main]
async fn main() {
    dotenv().ok();
    let config: Config = env_reader::read_config();
    {
        // create the table and populate the dummy data
        if !config.is_prod_env(){
            _ = init_db(&config).await;
        }
    }

    let conn = db_connection(&config).await.expect("db connection cannot be established");
    println!("{:?}", conn);
}
