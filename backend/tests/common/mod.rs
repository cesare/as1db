use as1db::{
    context::{Config, Context},
    repositories::RdbRepositoryFactory,
};
use sqlx::PgPool;

fn create_config() -> Config {
    envy::from_env::<Config>().unwrap()
}

pub fn create_context(pool: PgPool) -> Context {
    let config = create_config();
    Context {
        config,
        repositories: RdbRepositoryFactory::new(pool),
    }
}
