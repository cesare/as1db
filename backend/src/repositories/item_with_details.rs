use async_trait::async_trait;
use indoc::indoc;
use sqlx::{Executor, Pool, Postgres};

use crate::{
    errors::DatabaseError,
    models::{ItemId, ItemWithDetails},
    repositories::ItemWithDetailsRepository,
};

pub struct RdbItemWithDetailsRepository<'r> {
    pool: &'r Pool<Postgres>,
}

impl<'r> RdbItemWithDetailsRepository<'r> {
    pub fn new(pool: &'r Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> ItemWithDetailsRepository for RdbItemWithDetailsRepository<'r> {
    async fn list(&self) -> Result<Vec<ItemWithDetails>, DatabaseError> {
        list(self.pool).await
    }

    async fn find(&self, item_id: &ItemId) -> Result<ItemWithDetails, DatabaseError> {
        find(self.pool, item_id).await
    }
}

async fn list<'c, T>(executor: T) -> Result<Vec<ItemWithDetails>, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
    let statement = indoc! {"
        with categories_for_items as (
            select
                ic.item_id as item_id,
                array_agg(to_json(categories)) as categories
            from item_categories as ic
                inner join categories on ic.category_id = categories.id
            group by 1
        ),
        material_items_for_items as (
            select
                mi.item_id as item_id,
                array_agg(to_json(items)) as material_items
            from material_items as mi
                inner join items on mi.material_item_id = items.id
            group by 1
        ),
        material_categories_for_items as (
            select
                mc.item_id as item_id,
                array_agg(to_json(categories)) as material_categories
            from material_categories as mc
                inner join categories on mc.category_id = categories.id
            group by 1
        )

        select
            items.id,
            items.name,
            to_json(classes) as class,
            c.categories as categories,
            coalesce(mi.material_items, array[]::json[]) as material_items,
            coalesce(mc.material_categories, array[]::json[]) as material_categories
        from items
            inner join classes on items.class_id = classes.id
            inner join categories_for_items as c on items.id = c.item_id
            left outer join material_items_for_items as mi on items.id = mi.item_id
            left outer join material_categories_for_items as mc on items.id = mc.item_id
        order by 1
    "};
    let ids: Vec<ItemWithDetails> = sqlx::query_as(statement)
        .fetch_all(executor)
        .await
        .inspect_err(|e| log::error!("query failed: {:?}", e))?;
    Ok(ids)
}

async fn find<'c, T>(executor: T, item_id: &ItemId) -> Result<ItemWithDetails, DatabaseError>
where
    T: Executor<'c, Database = Postgres>,
{
    let statement = indoc! {"
        with categories_for_items as (
            select
                ic.item_id as item_id,
                array_agg(to_json(categories)) as categories
            from item_categories as ic
                inner join categories on ic.category_id = categories.id
            where ic.item_id = $1
            group by 1
        ),
        material_items_for_items as (
            select
                mi.item_id as item_id,
                array_agg(to_json(items)) as material_items
            from material_items as mi
                inner join items on mi.material_item_id = items.id
            where mi.item_id = $1
            group by 1
        ),
        material_categories_for_items as (
            select
                mc.item_id as item_id,
                array_agg(to_json(categories)) as material_categories
            from material_categories as mc
                inner join categories on mc.category_id = categories.id
            where mc.item_id = $1
            group by 1
        )

        select
            items.id,
            items.name,
            to_json(classes) as class,
            c.categories as categories,
            coalesce(mi.material_items, array[]::json[]) as material_items,
            coalesce(mc.material_categories, array[]::json[]) as material_categories
        from items
            inner join classes on items.class_id = classes.id
            inner join categories_for_items as c on items.id = c.item_id
            left outer join material_items_for_items as mi on items.id = mi.item_id
            left outer join material_categories_for_items as mc on items.id = mc.item_id
        where items.id = $1
    "};

    let item: ItemWithDetails = sqlx::query_as(statement)
        .bind(item_id)
        .fetch_one(executor)
        .await
        .inspect_err(|e| log::error!("Query failed: {:?}", e))?;
    Ok(item)
}
