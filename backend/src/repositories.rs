use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    errors::DatabaseError,
    models::{Category, CategoryId, Class, ClassId, Item, ItemId, ItemWithDetails},
    repositories::{
        category::RdbCategoryRepository, class::RdbClassRepository, item::RdbItemRepository,
        item_with_details::RdbItemWithDetailsRepository,
    },
};

mod category;
mod class;
mod item;
mod item_with_details;

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
    fn class<'a>(&self) -> Box<dyn ClassRepository + '_>;
    fn category<'a>(&self) -> Box<dyn CategoryRepository + '_>;
    fn item<'a>(&self) -> Box<dyn ItemRepository + '_>;
    fn item_with_details<'a>(&self) -> Box<dyn ItemWithDetailsRepository + '_>;
}

#[derive(Clone)]
pub struct RdbRepositoryFactory {
    pool: Pool<Postgres>,
}

impl RdbRepositoryFactory {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl RepositoryFactory for RdbRepositoryFactory {
    fn class<'a>(&self) -> Box<dyn ClassRepository + '_> {
        Box::new(RdbClassRepository::new(&self.pool))
    }

    fn category<'a>(&self) -> Box<dyn CategoryRepository + '_> {
        Box::new(RdbCategoryRepository::new(&self.pool))
    }

    fn item<'a>(&self) -> Box<dyn ItemRepository + '_> {
        Box::new(RdbItemRepository::new(&self.pool))
    }

    fn item_with_details<'a>(&self) -> Box<dyn ItemWithDetailsRepository + '_> {
        Box::new(RdbItemWithDetailsRepository::new(&self.pool))
    }
}
