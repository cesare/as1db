use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, middleware::Logger, web::Data};
use env_logger::Env;

mod context;
mod errors;
mod handlers;
mod models;
mod views;

use crate::context::Context;

fn build_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3001")
        .allowed_methods(vec!["POST", "GET", "DELETE", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let context = Context::load()?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(build_cors())
            .wrap(Logger::new(
                "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .app_data(Data::new(context.clone()))
            .configure(handlers::routes)
    });
    server.bind(("localhost", 3000))?.run().await?;
    Ok(())
}
