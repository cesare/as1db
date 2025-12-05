use actix_web::web::{ServiceConfig, scope};

mod categories;
mod classes;

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .service(scope("/categories").configure(categories::routes))
        .service(scope("/classes").configure(classes::routes));
}
