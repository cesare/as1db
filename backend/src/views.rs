use serde::Serialize;

use crate::models::{Class, ClassId};

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
