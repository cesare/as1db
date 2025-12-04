use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header};

fn build_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3001")
        .allowed_methods(vec!["POST", "GET", "DELETE", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(build_cors())
    });
    server.bind(("localhost", 3000))?.run().await?;
    Ok(())
}
