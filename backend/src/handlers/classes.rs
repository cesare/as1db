use actix_web::{
    HttpResponse,
    error::HttpError,
    web::{Data, ServiceConfig, get},
};

use crate::context::Context;

pub(super) fn routes(config: &mut ServiceConfig) {
    config.route("", get().to(index));
}

async fn index(_context: Data<Context>) -> Result<HttpResponse, HttpError> {
    todo!()
}
