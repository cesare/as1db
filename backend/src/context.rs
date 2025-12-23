use anyhow::Result;
use serde::Deserialize;
use sqlx::PgPool;

use crate::repositories::RdbRepositoryFactory;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub bind_address: String,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub repositories: RdbRepositoryFactory,
}

impl Context {
    pub fn load() -> Result<Self> {
        let config = envy::from_env::<Config>()?;
        let pool = PgPool::connect_lazy(&config.database_url)?;
        let repositories = RdbRepositoryFactory::new(pool);

        let context = Self {
            config,
            repositories,
        };
        Ok(context)
    }
}
