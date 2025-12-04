use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Context {
    pub pool: Pool<Postgres>,
}

impl Context {
    pub fn load() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect_lazy(&database_url)?;
        Ok(Self { pool })
    }
}
