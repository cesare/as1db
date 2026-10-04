use async_trait::async_trait;
use sqlx::{Executor, Pool, Postgres, Transaction};

use crate::{
    errors::DatabaseError,
    models::{Class, ClassId},
    repositories::{ClassRepository, TransactionalClassRepository},
};

pub struct RdbClassRepository<'r> {
    pool: &'r Pool<Postgres>,
}

impl<'r> RdbClassRepository<'r> {
    pub fn new(pool: &'r Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> ClassRepository for RdbClassRepository<'r> {
    async fn list(&self) -> Result<Vec<Class>, DatabaseError> {
        list(self.pool).await
    }

    async fn find(&self, class_id: &ClassId) -> Result<Class, DatabaseError> {
        find(self.pool, class_id).await
    }
}

pub struct TxClassRepository<'c> {
    tx: &'c mut Transaction<'c, Postgres>,
}

impl<'c> TxClassRepository<'c> {
    pub fn new(tx: &'c mut Transaction<'c, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'c> TransactionalClassRepository for TxClassRepository<'c> {
    async fn list(&mut self) -> Result<Vec<Class>, DatabaseError> {
        list(&mut **self.tx).await
    }

    async fn find(&mut self, class_id: &ClassId) -> Result<Class, DatabaseError> {
        find(&mut **self.tx, class_id).await
    }
}

async fn list<'c, T>(executor: T) -> Result<Vec<Class>, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
    let classes: Vec<Class> = sqlx::query_as("select id, name from classes order by id")
        .fetch_all(executor)
        .await?;
    Ok(classes)
}

async fn find<'c, T>(executor: T, class_id: &ClassId) -> Result<Class, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
    let class: Class = sqlx::query_as("select id, name from classes where id = $1")
        .bind(class_id)
        .fetch_one(executor)
        .await?;
    Ok(class)
}
