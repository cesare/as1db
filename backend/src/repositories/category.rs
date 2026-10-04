use async_trait::async_trait;
use sqlx::{Executor, Pool, Postgres, Transaction};

use crate::{
    errors::DatabaseError,
    models::{Category, CategoryId},
    repositories::{CategoryRepository, TransactionalCategoryRepository},
};

pub struct RdbCategoryRepository<'r> {
    pool: &'r Pool<Postgres>,
}

impl<'r> RdbCategoryRepository<'r> {
    pub fn new(pool: &'r Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> CategoryRepository for RdbCategoryRepository<'r> {
    async fn list(&self) -> Result<Vec<Category>, DatabaseError> {
        list(self.pool).await
    }

    async fn find(&self, category_id: &CategoryId) -> Result<Category, DatabaseError> {
        find(self.pool, category_id).await
    }
}

async fn list<'c, T>(executor: T) -> Result<Vec<Category>, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
    let categories: Vec<Category> = sqlx::query_as("select id, name from categories order by id")
        .fetch_all(executor)
        .await?;
    Ok(categories)
}

async fn find<'c, T>(executor: T, category_id: &CategoryId) -> Result<Category, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
    let category: Category = sqlx::query_as("select id, name from categories where id = $1")
        .bind(category_id)
        .fetch_one(executor)
        .await?;
    Ok(category)
}

pub struct TxCategoryRepository<'c> {
    tx: &'c mut Transaction<'c, Postgres>,
}

impl<'c> TxCategoryRepository<'c> {
    pub fn new(tx: &'c mut Transaction<'c, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'c> TransactionalCategoryRepository for TxCategoryRepository<'c> {
    async fn list(&mut self) -> Result<Vec<Category>, DatabaseError> {
        list(&mut **self.tx).await
    }

    async fn find(&mut self, category_id: &CategoryId) -> Result<Category, DatabaseError> {
        find(&mut **self.tx, category_id).await
    }
}
