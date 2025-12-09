use serde::Serialize;

use crate::models::{Category, CategoryId, Class, ClassId, Item, ItemId, ItemWithDetails};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemWithDetailsView<'a> {
    pub id: &'a ItemId,
    pub name: &'a String,
    pub class: ClassView<'a>,
    pub categories: Vec<CategoryView<'a>>,
    pub material_items: Vec<ItemView<'a>>,
    pub material_categories: Vec<CategoryView<'a>>,
}

impl<'a> ItemWithDetailsView<'a> {
    pub fn new(item: &'a ItemWithDetails) -> Self {
        Self {
            id: &item.id,
            name: &item.name,
            class: ClassView::new(&item.class),
            categories: item.categories.iter().map(|c| CategoryView::new(&c)).collect(),
            material_items: item.material_items.iter().map(|i| ItemView::new(&i)).collect(),
            material_categories: item.material_categories.iter().map(|c| CategoryView::new(&c)).collect(),
        }
    }
}
