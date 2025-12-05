use actix_web::{
    HttpResponse,
    web::{Data, ServiceConfig, get},
};
use serde_json::json;

use crate::{context::Context, errors::PerRequestError, models::ItemResources, views::ItemView};

pub(super) fn routes(config: &mut ServiceConfig) {
    config.route("", get().to(index));
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
