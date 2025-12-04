use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web::Data};

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
    let context = Context::load()?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(build_cors())
            .app_data(Data::new(context.clone()))
            .configure(handlers::routes)
    });
    server.bind(("localhost", 3000))?.run().await?;
    Ok(())
}
