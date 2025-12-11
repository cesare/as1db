use actix_web::{
    HttpResponse,
    web::{Data, Path, ServiceConfig, get},
};
use serde_json::json;

use crate::{
    context::Context,
    errors::PerRequestError,
    models::{ClassId, ClassResources, ItemResources},
    views::{ClassView, ItemView},
};

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("/{id}", get().to(show));
}

async fn index(context: Data<Context>) -> Result<HttpResponse, PerRequestError> {
    let resources = ClassResources::new(&context);
    let classes = resources.list().await?;
    let response_json = json!({
        "classes": classes.iter().map(ClassView::new).collect::<Vec<ClassView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn show(
    context: Data<Context>,
    path: Path<ClassId>,
) -> Result<HttpResponse, PerRequestError> {
    let class_id = path.into_inner();
    let class = ClassResources::new(&context).find(&class_id).await?;

    let resources = ItemResources::new(&context);
    let items = resources.list_by_class(&class).await?;

    let response_json = json!({
        "class": ClassView::new(&class),
        "items": items.iter().map(ItemView::new).collect::<Vec<ItemView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
