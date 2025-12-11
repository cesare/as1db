use indoc::indoc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

use crate::{context::Context, errors::DatabaseError};

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct ClassId(i32);

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct Class {
    pub id: ClassId,
    pub name: String,
}

pub struct ClassResources<'a> {
    context: &'a Context,
}

impl<'a> ClassResources<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub async fn list(&self) -> Result<Vec<Class>, DatabaseError> {
        let classes: Vec<Class> = sqlx::query_as("select id, name from classes order by id")
            .fetch_all(&self.context.pool)
            .await?;
        Ok(classes)
    }

    pub async fn find(&self, class_id: &ClassId) -> Result<Class, DatabaseError> {
        let class: Class = sqlx::query_as("select id, name from classes where id = $1")
            .bind(class_id)
            .fetch_one(&self.context.pool)
            .await?;
        Ok(class)
    }
}

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct CategoryId(i32);

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
}

pub struct CategoryResources<'a> {
    context: &'a Context,
}

impl<'a> CategoryResources<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub async fn list(&self) -> Result<Vec<Category>, DatabaseError> {
        let categories: Vec<Category> =
            sqlx::query_as("select id, name from categories order by id")
                .fetch_all(&self.context.pool)
                .await?;
        Ok(categories)
    }

    pub async fn find(&self, category_id: &CategoryId) -> Result<Category, DatabaseError> {
        let category: Category = sqlx::query_as("select id, name from categories where id = $1")
            .bind(category_id)
            .fetch_one(&self.context.pool)
            .await?;
        Ok(category)
    }
}

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ItemId(i32);

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct Item {
    pub id: ItemId,
    pub class_id: ClassId,
    pub name: String,
}

pub struct ItemResources<'a> {
    context: &'a Context,
}

impl<'a> ItemResources<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub async fn list(&self) -> Result<Vec<Item>, DatabaseError> {
        let items: Vec<Item> = sqlx::query_as("select id, class_id, name from items order by id")
            .fetch_all(&self.context.pool)
            .await?;
        Ok(items)
    }

    pub async fn list_by_class(&self, class: &Class) -> Result<Vec<Item>, DatabaseError> {
        let items: Vec<Item> =
            sqlx::query_as("select id, class_id, name from items where class_id = $1 order by id")
                .bind(class.id)
                .fetch_all(&self.context.pool)
                .await?;
        Ok(items)
    }

    pub async fn list_by_category(&self, category: &Category) -> Result<Vec<Item>, DatabaseError> {
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
            .fetch_all(&self.context.pool)
            .await
            .inspect_err(|e| log::error!("Query failed: {:?}", e))?;
        Ok(items)
    }
}

#[derive(Clone, Deserialize, FromRow)]
pub struct ItemCategory {
    pub item_id: ItemId,
    pub category_id: CategoryId,
}

pub struct ItemCategoryResources<'a> {
    context: &'a Context,
}

impl<'a> ItemCategoryResources<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub async fn list(&self) -> Result<Vec<ItemCategory>, DatabaseError> {
        let ics: Vec<ItemCategory> =
            sqlx::query_as("select item_id, category_id from item_categories order by item_id")
                .fetch_all(&self.context.pool)
                .await?;
        Ok(ics)
    }
}

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct MaterialItem {
    pub item_id: ItemId,
    pub material_item_id: ItemId,
}

pub struct MaterialItemResources<'a> {
    context: &'a Context,
}

impl<'a> MaterialItemResources<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub async fn list(&self) -> Result<Vec<MaterialItem>, DatabaseError> {
        let mis: Vec<MaterialItem> =
            sqlx::query_as("select item_id, material_item_id from material_items order by item_id")
                .fetch_all(&self.context.pool)
                .await?;
        Ok(mis)
    }
}

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct MaterialCategory {
    pub item_id: ItemId,
    pub category_id: CategoryId,
}

pub struct MaterialCategoryResources<'a> {
    context: &'a Context,
}

impl<'a> MaterialCategoryResources<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub async fn list(&self) -> Result<Vec<MaterialCategory>, DatabaseError> {
        let mcs: Vec<MaterialCategory> =
            sqlx::query_as("select item_id, category_id from material_categories order by item_id")
                .fetch_all(&self.context.pool)
                .await?;
        Ok(mcs)
    }
}

#[derive(Clone, Deserialize, FromRow)]
pub struct ItemWithDetails {
    pub id: ItemId,
    pub name: String,
    pub class: Json<Class>,
    pub categories: Vec<Json<Category>>,
    pub material_items: Vec<Json<Item>>,
    pub material_categories: Vec<Json<Category>>,
}

pub struct ItemWithDetailsResources<'a> {
    context: &'a Context,
}

impl<'a> ItemWithDetailsResources<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    pub async fn list(&self) -> Result<Vec<ItemWithDetails>, DatabaseError> {
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
            .fetch_all(&self.context.pool)
            .await
            .inspect_err(|e| log::error!("query failed: {:?}", e))?;
        Ok(ids)
    }

    pub async fn find(&self, item_id: &ItemId) -> Result<ItemWithDetails, DatabaseError> {
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
            .fetch_one(&self.context.pool)
            .await
            .inspect_err(|e| log::error!("Query failed: {:?}", e))?;
        Ok(item)
    }
}
