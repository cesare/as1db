use actix_web::{
    HttpResponse,
    web::{Data, Path, ServiceConfig, get},
};
use serde_json::json;

use crate::{
    context::Context,
    errors::PerRequestError,
    models::{CategoryId, CategoryResources, ItemResources},
    views::{CategoryView, ItemView},
};

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("/{id}", get().to(show));
}

async fn index(context: Data<Context>) -> Result<HttpResponse, PerRequestError> {
    let resources = CategoryResources::new(&context);
    let categories = resources.list().await?;
    let response_json = json!({
        "categories": categories.iter().map(CategoryView::new).collect::<Vec<CategoryView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn show(
    context: Data<Context>,
    path: Path<CategoryId>,
) -> Result<HttpResponse, PerRequestError> {
    let category_id = path.into_inner();
    let category = CategoryResources::new(&context).find(&category_id).await?;

    let resources = ItemResources::new(&context);
    let items = resources.list_by_category(&category).await?;

    let response_json = json!({
        "category": CategoryView::new(&category),
        "items": items.iter().map(ItemView::new).collect::<Vec<ItemView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
