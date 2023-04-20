mod models;
mod env_reader;

use models::db_connection;
use dotenv::dotenv;
use env_reader::Config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = env_reader::read_config();

    let conn = db_connection(&config).await.expect("db connection cannot be established");
    println!("{:?}", conn);
}
