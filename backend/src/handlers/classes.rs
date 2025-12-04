use actix_web::{
    HttpResponse,
    web::{Data, ServiceConfig, get},
};
use serde_json::json;

use crate::{context::Context, errors::PerRequestError, models::ClassResources, views::ClassView};

pub(super) fn routes(config: &mut ServiceConfig) {
    config.route("", get().to(index));
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
