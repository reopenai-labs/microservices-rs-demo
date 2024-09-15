use std::path::Path;

mod config;
mod db;
use anyhow::Ok;
pub use config::Environment;
pub use db::DB;

use crate::log;

pub struct Context {
    environment: Environment,
    // datasource: DataSource,
    db: DB,
}

impl Context {
    pub fn get_environment(&'static self) -> &Environment {
        &self.environment
    }

    pub fn get_datasource(&'static self) -> &'static DB {
        &self.db
    }
}

impl Context {
    pub async fn new_static_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<&'static Self> {
        let instance = Self::new_from_path(path).await?;
        Ok(Box::leak(Box::new(instance)))
    }

    pub async fn new_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let environment = Environment::new(path)?;
        log::info!("[Application]配置信息初始化完成");
        // let datasource = db::DataSource::new(&environment).await?;
        let db = db::DB::new(&environment).await?;
        log::info!("[Application]数据库初始化完成");
        let instance = Context {
            environment,
            db,
        };
        Ok(instance)
    }
}
