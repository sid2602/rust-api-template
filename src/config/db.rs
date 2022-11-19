use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use super::config::{Config};

pub type DbPool = Pool<Postgres>;

pub struct DbConnection {
    pub pool: Pool<Postgres>
}

impl DbConnection{
    pub async fn new() -> DbConnection {
        let config = Config::new();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(format!("postgres://{}:{}@{}/{}", config.db_user,config.db_password,config.db_port,config.db_name).as_str()).await.expect("pool failed");

        return DbConnection{
            pool
        };
    }
}
