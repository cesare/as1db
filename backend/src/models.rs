use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct ClassId(i32);

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct Class {
    pub id: ClassId,
    pub name: String,
}

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct CategoryId(i32);

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
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

#[derive(Clone, Deserialize, FromRow)]
pub struct ItemCategory {
    pub item_id: ItemId,
    pub category_id: CategoryId,
}

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct MaterialItem {
    pub item_id: ItemId,
    pub material_item_id: ItemId,
}

#[derive(Clone, Deserialize, FromRow, sqlx::Type)]
pub struct MaterialCategory {
    pub item_id: ItemId,
    pub category_id: CategoryId,
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
