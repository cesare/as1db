use std::marker::PhantomData;

use async_trait::async_trait;
use sqlx::{Executor, Postgres};

use crate::{
    errors::DatabaseError,
    models::{Category, CategoryId},
    repositories::CategoryRepository,
};

pub struct RdbCategoryRepository<'a, T: Executor<'a>> {
    executor: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RdbCategoryRepository<'a, T>
where
    T: Executor<'a, Database = Postgres> + Copy + Sync,
{
    pub fn new(executor: T) -> Self {
        Self {
            executor,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<'a, T> CategoryRepository for RdbCategoryRepository<'a, T>
where
    T: Executor<'a, Database = Postgres> + Copy + Sync,
{
    async fn list(&self) -> Result<Vec<Category>, DatabaseError> {
        let categories: Vec<Category> =
            sqlx::query_as("select id, name from categories order by id")
                .fetch_all(self.executor)
                .await?;
        Ok(categories)
    }

    async fn find(&self, category_id: &CategoryId) -> Result<Category, DatabaseError> {
        let category: Category = sqlx::query_as("select id, name from categories where id = $1")
            .bind(category_id)
            .fetch_one(self.executor)
            .await?;
        Ok(category)
    }
}
