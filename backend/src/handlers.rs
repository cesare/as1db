use actix_web::web::{ServiceConfig, scope};

mod categories;
mod classes;
mod items;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(scope("/categories").configure(categories::routes))
        .service(scope("/classes").configure(classes::routes))
        .service(scope("/items").configure(items::routes));
}
