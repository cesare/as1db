use anyhow::Result;
use serde::Deserialize;
use sqlx::{PgPool, Pool, Postgres};

#[derive(Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub pool: Pool<Postgres>,
}

impl Context {
    pub fn load() -> Result<Self> {
        let config = envy::from_env::<Config>()?;
        let pool = PgPool::connect_lazy(&config.database_url)?;

        let context = Self { config, pool };
        Ok(context)
    }
}
