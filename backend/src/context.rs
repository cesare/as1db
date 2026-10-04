use anyhow::Result;
use serde::Deserialize;
use sqlx::PgPool;

use crate::repositories::RdbRepository;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub bind_address: String,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub repository: RdbRepository,
}

impl Context {
    pub fn load() -> Result<Self> {
        let config = envy::from_env::<Config>()?;
        let pool = PgPool::connect_lazy(&config.database_url)?;
        let repository = RdbRepository::new(pool);

        let context = Self { config, repository };
        Ok(context)
    }
}
