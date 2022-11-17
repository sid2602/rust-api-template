use std::{env};

extern crate dotenv;
pub struct Config {
    pub db_user: String,
    pub db_password: String,
    pub db_port: String,
    pub db_name: String
}

impl Config {
    pub fn new() -> Config {

        let db_user = env::var("DB_USER").expect("$DB_USER is not set");
        let db_password = env::var("DB_PASSWORD").expect("$DB_PASSWORD is not set");
        let db_port = env::var("DB_PORT").expect("$DB_PORT is not set");
        let db_name = env::var("DB_NAME").expect("$DB_NAME is not set");

        Config {
            db_user,
            db_password,
            db_port,
            db_name,
        }
    }
}