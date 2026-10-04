use async_trait::async_trait;
use indoc::indoc;
use sqlx::{Executor, Pool, Postgres};

use crate::{
    errors::DatabaseError,
    models::{Category, Class, Item},
    repositories::ItemRepository,
};

pub struct RdbItemRepository<'r> {
    pool: &'r Pool<Postgres>,
}

impl<'r> RdbItemRepository<'r> {
    pub fn new(pool: &'r Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> ItemRepository for RdbItemRepository<'r> {
    async fn list(&self) -> Result<Vec<Item>, DatabaseError> {
        list(self.pool).await
    }

    async fn list_by_class(&self, class: &Class) -> Result<Vec<Item>, DatabaseError> {
        list_by_class(self.pool, class).await
    }

    async fn list_by_category(&self, category: &Category) -> Result<Vec<Item>, DatabaseError> {
        list_by_category(self.pool, category).await
    }
}

async fn list<'c, T>(executor: T) -> Result<Vec<Item>, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
    let items: Vec<Item> = sqlx::query_as("select id, class_id, name from items order by id")
        .fetch_all(executor)
        .await?;
    Ok(items)
}

async fn list_by_class<'c, T>(executor: T, class: &Class) -> Result<Vec<Item>, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
    let items: Vec<Item> =
        sqlx::query_as("select id, class_id, name from items where class_id = $1 order by id")
            .bind(class.id)
            .fetch_all(executor)
            .await?;
    Ok(items)
}

async fn list_by_category<'c, T>(
    executor: T,
    category: &Category,
) -> Result<Vec<Item>, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
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
        .fetch_all(executor)
        .await
        .inspect_err(|e| log::error!("Query failed: {:?}", e))?;
    Ok(items)
}
