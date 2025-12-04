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
