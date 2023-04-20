use dotenv::dotenv;
use std::env;

pub fn read_config() -> Config {
    dotenv().ok();

    let env = env::var("env").expect("error reading the env variable");

    let db_host = env::var("db_host").expect("error reading the db_host variable");
    let db_user = env::var("db_user").expect("error reading the db_user variable");
    let db_pass = env::var("db_pass").expect("error reading the db_pass variable");
    let db_name = env::var("db_name").expect("error reading the db_name variable");

    Config { db_host, db_user, db_pass, db_name, env, ..Default::default()}
}


#[derive(Default, Debug)]
pub struct Config {
    pub db_host: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
    pub env: String,
    pub debug: bool,
}

impl Config {
    pub fn is_prod_env(&self) -> bool {
        if self.env.to_lowercase() == "production" {
            return true
        }
        false
    }
}
