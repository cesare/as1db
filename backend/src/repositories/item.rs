use std::marker::PhantomData;

use async_trait::async_trait;
use indoc::indoc;
use sqlx::{Executor, Postgres};

use crate::{
    errors::DatabaseError,
    models::{Category, Class, Item},
    repositories::ItemRepository,
};

pub struct RdbItemRepository<'a, T: Executor<'a>> {
    executor: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RdbItemRepository<'a, T>
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
impl<'a, T> ItemRepository for RdbItemRepository<'a, T>
where
    T: Executor<'a, Database = Postgres> + Copy + Sync,
{
    async fn list(&self) -> Result<Vec<Item>, DatabaseError> {
        let items: Vec<Item> = sqlx::query_as("select id, class_id, name from items order by id")
            .fetch_all(self.executor)
            .await?;
        Ok(items)
    }

    async fn list_by_class(&self, class: &Class) -> Result<Vec<Item>, DatabaseError> {
        let items: Vec<Item> =
            sqlx::query_as("select id, class_id, name from items where class_id = $1 order by id")
                .bind(class.id)
                .fetch_all(self.executor)
                .await?;
        Ok(items)
    }

    async fn list_by_category(&self, category: &Category) -> Result<Vec<Item>, DatabaseError> {
        let statement = indoc! {"
            select
                items.id as id,
                items.class_id as class_id,
                items.name as name
            from item_categories as ic
                inner join items on ic.item_id = items.id
            where ic.category_id = $1
            order by 1
        "};
        let items: Vec<Item> = sqlx::query_as(statement)
            .bind(&category.id)
            .fetch_all(self.executor)
            .await
            .inspect_err(|e| log::error!("Query failed: {:?}", e))?;
        Ok(items)
    }
}
