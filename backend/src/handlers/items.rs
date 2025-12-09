use actix_web::{
    HttpResponse,
    web::{Data, ServiceConfig, get},
};
use serde_json::json;

use crate::{context::Context, errors::PerRequestError, models::{ItemResources, ItemWithDetailsResources}, views::{ItemView, ItemWithDetailsView}};

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("/with_details", get().to(index_with_details));
}

async fn index(context: Data<Context>) -> Result<HttpResponse, PerRequestError> {
    let resources = ItemResources::new(&context);
    let items = resources.list().await?;
    let response_json = json!({
        "items": items.iter().map(ItemView::new).collect::<Vec<ItemView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn index_with_details(context: Data<Context>) -> Result<HttpResponse, PerRequestError> {
    let resources = ItemWithDetailsResources::new(&context);
    let items = resources.list().await?;
    let response_json = json!({
        "items": items.iter().map(ItemWithDetailsView::new).collect::<Vec<ItemWithDetailsView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
