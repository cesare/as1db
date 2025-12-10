use crate::models::{Category, CategoryId, Class, ClassId, Item, ItemId};

pub struct ClassSet {
    classes: Vec<Class>,
}

impl ClassSet {
    pub fn new(classes: Vec<Class>) -> Self {
        Self { classes }
    }

    pub fn find_by_id(&self, class_id: ClassId) -> Option<&Class> {
        self.classes.iter().find(|c| c.id == class_id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|c| c.name == name)
    }
}

pub struct CategorySet {
    categories: Vec<Category>,
}

impl CategorySet {
    pub fn new(categories: Vec<Category>) -> Self {
        Self { categories }
    }

    pub fn find_by_id(&self, category_id: CategoryId) -> Option<&Category> {
        self.categories.iter().find(|c| c.id == category_id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Category> {
        self.categories.iter().find(|c| c.name == name)
    }
}

pub struct ItemSet {
    items: Vec<Item>,
}

impl ItemSet {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    pub fn find_by_id(&self, item_id: ItemId) -> Option<&Item> {
        self.items.iter().find(|i| i.id == item_id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|i| i.name == name)
    }
}
