use actix_web::{
    HttpResponse,
    web::{Data, Path, ServiceConfig, get},
};
use serde_json::json;

use crate::{
    context::Context,
    errors::PerRequestError,
    models::ItemId,
    repositories::RepositoryFactory,
    views::{ItemView, ItemWithDetailsView},
};

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("/with_details", get().to(index_with_details))
        .route("/{id}", get().to(show));
}

async fn index(context: Data<Context>) -> Result<HttpResponse, PerRequestError> {
    let repository = context.repositories.item();
    let items = repository.list().await?;
    let response_json = json!({
        "items": items.iter().map(ItemView::new).collect::<Vec<ItemView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn index_with_details(context: Data<Context>) -> Result<HttpResponse, PerRequestError> {
    let repository = context.repositories.item_with_details();
    let items = repository.list().await?;
    let response_json = json!({
        "items": items.iter().map(ItemWithDetailsView::new).collect::<Vec<ItemWithDetailsView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn show(context: Data<Context>, path: Path<ItemId>) -> Result<HttpResponse, PerRequestError> {
    let item_id = path.into_inner();

    let repository = context.repositories.item_with_details();
    let item = repository.find(&item_id).await?;

    let response_json = json!({
        "item": ItemWithDetailsView::new(&item),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
