use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{context::Context, errors::DatabaseError};

#[derive(Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ClassId(i32);

#[derive(Clone, Deserialize, FromRow)]
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
}

#[derive(Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct CategoryId(i32);

#[derive(Clone, Deserialize, FromRow)]
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
}

#[derive(Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ItemId(i32);

#[derive(Clone, Deserialize, FromRow)]
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
        let ics: Vec<ItemCategory> = sqlx::query_as("select item_id, category_id from item_categories order by item_id")
            .fetch_all(&self.context.pool)
            .await?;
        Ok(ics)
    }
}

#[derive(Clone, Deserialize, FromRow)]
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
        let mis: Vec<MaterialItem> = sqlx::query_as("select item_id, material_item_id from material_items order by item_id")
            .fetch_all(&self.context.pool)
            .await?;
        Ok(mis)
    }
}
