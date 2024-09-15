use std::time::Duration;

use anyhow::Ok;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct DB {
    db: DatabaseConnection,
}

impl DB {
    pub fn get_conn(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl DB {
    pub async fn new(env: &super::Environment) -> anyhow::Result<DB> {
        let username = env
            .get_string("application.datasource.username")
            .unwrap_or_else(|| "postgres".to_string());
        let password = env
            .get_string("application.datasource.password")
            .unwrap_or_else(|| "postgres".to_string());
        let host = env
            .get_string("application.datasource.host")
            .unwrap_or_else(|| "127.0.0.1:5432".to_string());
        let database = env
            .get_string("application.datasource.database")
            .unwrap_or_else(|| "postgres".to_string());
        let schema = env
            .get_string("application.datasource.schema")
            .unwrap_or_else(|| "public".to_string());
        let database_url = format!("postgres://{username}:{password}@{host}/{database}");
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(30))
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(30))
            .max_lifetime(Duration::from_secs(30))
            .sqlx_logging(true)
            // .sqlx_logging_level(log::LevelFilter::Info)
            .connect_lazy(true)
            .set_schema_search_path(schema);
        let db = Database::connect(opt).await?;
        Ok(DB { db })
    }
}
