use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
    });
    server.bind(("localhost", 3000))?.run().await?;
    Ok(())
}
