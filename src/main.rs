pub mod logsaveservice{
    tonic::include_proto!("protos");
}
use logsaveservice::log_save_service_server::LogSaveServiceServer;

mod models;
mod env_reader;
mod log_grpc;
use models::{init_db, db_connection};
use dotenv::dotenv;
use env_reader::Config;
use tonic::transport::Server;
use log_grpc::newgrpchandler;

/// The main function is the entry point of the Rust program. 
/// It loads the environment variables from the .env file, reads the configuration values, 
/// establishes a database connection using the supplied configuration and prints the connection details.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config: Config = env_reader::read_config();
    {
        // create the table and populate the dummy data
        if !config.is_prod_env(){
            _ = init_db(&config).await;
        }
    }

    let server_add = format!("{}:{}", &config.srv_add, &config.srv_port).parse().unwrap_or_else(|e| panic!("Error defining address {}", e));

    let conn = db_connection(&config).await.expect("db connection cannot be established");

    let log_srv = LogSaveServiceServer::new(newgrpchandler(conn));

    println!("now listening at {}:{}", &config.srv_add, &config.srv_port);
    Server::builder()
        .add_service(log_srv)
        .serve(server_add)
        .await?;

    Ok(())
}
