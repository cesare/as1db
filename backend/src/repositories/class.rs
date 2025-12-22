use std::marker::PhantomData;

use async_trait::async_trait;
use sqlx::{Executor, Postgres};

use crate::{
    errors::DatabaseError,
    models::{Class, ClassId},
    repositories::ClassRepository,
};

pub struct RdbClassRepository<'a, T: Executor<'a>> {
    executor: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RdbClassRepository<'a, T>
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
impl<'a, T> ClassRepository for RdbClassRepository<'a, T>
where
    T: Executor<'a, Database = Postgres> + Copy + Sync,
{
    async fn list(&self) -> Result<Vec<Class>, DatabaseError> {
        let classes: Vec<Class> = sqlx::query_as("select id, name from classes order by id")
            .fetch_all(self.executor)
            .await?;
        Ok(classes)
    }

    async fn find(&self, class_id: &ClassId) -> Result<Class, DatabaseError> {
        let class: Class = sqlx::query_as("select id, name from classes where id = $1")
            .bind(class_id)
            .fetch_one(self.executor)
            .await?;
        Ok(class)
    }
}
