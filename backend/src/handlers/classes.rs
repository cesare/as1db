use actix_web::{
    HttpResponse,
    web::{Data, Path, ServiceConfig, get},
};
use serde_json::json;

use crate::{
    context::Context,
    errors::PerRequestError,
    models::ClassId,
    repositories::RepositoryFactory,
    views::{ClassView, ItemView},
};

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("/{id}", get().to(show));
}

async fn index(context: Data<Context>) -> Result<HttpResponse, PerRequestError> {
    let repository = context.repositories.class();
    let classes = repository.list().await?;
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
    let class = context.repositories.class().find(&class_id).await?;

    let repository = context.repositories.item();
    let items = repository.list_by_class(&class).await?;

    let response_json = json!({
        "class": ClassView::new(&class),
        "items": items.iter().map(ItemView::new).collect::<Vec<ItemView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
