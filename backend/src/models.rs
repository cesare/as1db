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
