use serde::Serialize;

use crate::models::{Category, CategoryId, Class, ClassId, Item, ItemId};

#[derive(Serialize)]
#[serde(rename = "camelCase")]
pub struct ClassView<'a> {
    id: &'a ClassId,
    name: &'a String,
}

impl<'a> ClassView<'a> {
    pub fn new(class: &'a Class) -> Self {
        Self {
            id: &class.id,
            name: &class.name,
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "camelCase")]
pub struct CategoryView<'a> {
    id: &'a CategoryId,
    name: &'a String,
}

impl<'a> CategoryView<'a> {
    pub fn new(category: &'a Category) -> Self {
        Self {
            id: &category.id,
            name: &category.name,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemView<'a> {
    id: &'a ItemId,
    class_id: &'a ClassId,
    name: &'a String,
}

impl<'a> ItemView<'a> {
    pub fn new(item: &'a Item) -> Self {
        Self {
            id: &item.id,
            class_id: &item.class_id,
            name: &item.name,
        }
    }
}
