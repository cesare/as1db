use actix_web::web::{ServiceConfig, scope};

mod classes;

pub(super) fn routes(config: &mut ServiceConfig) {
    config.service(scope("/classes").configure(classes::routes));
}
