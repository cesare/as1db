use async_trait::async_trait;

use crate::{errors::DatabaseError, models::{Category, CategoryId, Class, ClassId, Item, ItemId, ItemWithDetails}};

#[async_trait]
pub trait ClassRepository {
    async fn list(&self) -> Result<Vec<Class>, DatabaseError>;
    async fn find(&self, class_id: &ClassId) -> Result<Class, DatabaseError>;
}

#[async_trait]
pub trait CategoryRepository {
    async fn list(&self) -> Result<Vec<Category>, DatabaseError>;
    async fn find(&self, category_id: &CategoryId) -> Result<Category, DatabaseError>;
}

#[async_trait]
pub trait ItemRepository {
    async fn list(&self) -> Result<Vec<Item>, DatabaseError>;
    async fn list_by_class(&self, class: &Class) -> Result<Vec<Item>, DatabaseError>;
    async fn list_by_category(&self, category: &Category) -> Result<Vec<Item>, DatabaseError>;
}

#[async_trait]
pub trait ItemWithDetailsRepository {
    async fn list(&self) -> Result<Vec<ItemWithDetails>, DatabaseError>;
    async fn find(&self, item_id: &ItemId) -> Result<ItemWithDetails, DatabaseError>;
}

pub trait RepositoryFactory {
    fn class(&self) -> dyn ClassRepository;
    fn category(&self) -> dyn CategoryRepository;
    fn item(&self) -> dyn ItemRepository;
    fn item_with_details(&self) -> dyn ItemWithDetailsRepository;
}
