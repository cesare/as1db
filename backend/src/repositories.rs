use async_trait::async_trait;
use sqlx::{Pool, Postgres, Transaction};

use crate::{
    errors::DatabaseError,
    models::{Category, CategoryId, Class, ClassId, Item, ItemId, ItemWithDetails},
    repositories::{
        category::{RdbCategoryRepository, TxCategoryRepository},
        class::{RdbClassRepository, TxClassRepository},
        item::RdbItemRepository,
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

#[async_trait]
pub trait Repository<'r> {
    type ClassRepository: ClassRepository;
    type CategoryRepository: CategoryRepository;
    type ItemRepository: ItemRepository;
    type ItemWithDetailsRepository: ItemWithDetailsRepository;
    type TransactionalRepository: TransactionalRepository<'r>;

    fn class(&'r self) -> Self::ClassRepository;
    fn category(&'r self) -> Self::CategoryRepository;
    fn item(&'r self) -> Self::ItemRepository;
    fn item_with_details(&'r self) -> Self::ItemWithDetailsRepository;

    async fn begin(&'r self) -> Result<Self::TransactionalRepository, DatabaseError>;
}

#[derive(Clone)]
pub struct RdbRepository {
    pool: Pool<Postgres>,
}

impl RdbRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool }
    }
}

#[async_trait]
impl<'r> Repository<'r> for RdbRepository {
    type ClassRepository = RdbClassRepository<'r>;
    type CategoryRepository = RdbCategoryRepository<'r>;
    type ItemRepository = RdbItemRepository<'r>;
    type ItemWithDetailsRepository = RdbItemWithDetailsRepository<'r>;
    type TransactionalRepository = TxRepository<'r>;

    fn class(&'r self) -> Self::ClassRepository {
        RdbClassRepository::new(&self.pool)
    }

    fn category(&'r self) -> Self::CategoryRepository {
        RdbCategoryRepository::new(&self.pool)
    }

    fn item(&'r self) -> Self::ItemRepository {
        RdbItemRepository::new(&self.pool)
    }

    fn item_with_details(&'r self) -> Self::ItemWithDetailsRepository {
        RdbItemWithDetailsRepository::new(&self.pool)
    }

    async fn begin(&'r self) -> Result<Self::TransactionalRepository, DatabaseError> {
        TxRepository::create(&self.pool).await
    }
}

#[async_trait]
pub trait TransactionalClassRepository {
    async fn list(&mut self) -> Result<Vec<Class>, DatabaseError>;
    async fn find(&mut self, class_id: &ClassId) -> Result<Class, DatabaseError>;
}

#[async_trait]
pub trait TransactionalCategoryRepository {
    async fn list(&mut self) -> Result<Vec<Category>, DatabaseError>;
    async fn find(&mut self, category_id: &CategoryId) -> Result<Category, DatabaseError>;
}

#[async_trait]
pub trait TransactionalRepository<'r> {
    type ClassRepository: TransactionalClassRepository;
    type CategoryRepository: TransactionalCategoryRepository;

    fn class(&'r mut self) -> Self::ClassRepository;
    fn category(&'r mut self) -> Self::CategoryRepository;

    async fn rollback(self) -> Result<(), DatabaseError>;
    async fn commit(self) -> Result<(), DatabaseError>;
}

pub struct TxRepository<'c> {
    tx: Transaction<'c, Postgres>,
}

impl<'c> TxRepository<'c> {
    async fn create(pool: &Pool<Postgres>) -> Result<Self, DatabaseError> {
        let tx = pool.begin().await?;
        let repository = TxRepository { tx };
        Ok(repository)
    }
}

#[async_trait]
impl<'c> TransactionalRepository<'c> for TxRepository<'c> {
    type ClassRepository = TxClassRepository<'c>;
    type CategoryRepository = TxCategoryRepository<'c>;

    fn class(&'c mut self) -> Self::ClassRepository {
        TxClassRepository::new(&mut self.tx)
    }

    fn category(&'c mut self) -> Self::CategoryRepository {
        TxCategoryRepository::new(&mut self.tx)
    }

    async fn rollback(self) -> Result<(), DatabaseError> {
        let _ = self.tx.rollback().await?;
        Ok(())
    }

    async fn commit(self) -> Result<(), DatabaseError> {
        let _ = self.tx.commit().await?;
        Ok(())
    }
}
