mod models;
mod env_reader;

use models::db_connection;
use dotenv::dotenv;
use env_reader::Config;

/// The main function is the entry point of the Rust program. 
/// It loads the environment variables from the .env file, reads the configuration values, 
/// establishes a database connection using the supplied configuration and prints the connection details.
#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = env_reader::read_config();

    let conn = db_connection(&config).await.expect("db connection cannot be established");
    println!("{:?}", conn);
}
