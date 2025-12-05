use actix_web::{
    HttpResponse,
    web::{Data, ServiceConfig, get},
};
use serde_json::json;

use crate::{
    context::Context, errors::PerRequestError, models::CategoryResources, views::CategoryView,
};

pub(super) fn routes(config: &mut ServiceConfig) {
    config.route("", get().to(index));
}

async fn index(context: Data<Context>) -> Result<HttpResponse, PerRequestError> {
    let resources = CategoryResources::new(&context);
    let classes = resources.list().await?;
    let response_json = json!({
        "categories": classes.iter().map(CategoryView::new).collect::<Vec<CategoryView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
