use actix_web::{middleware, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Initialize the logger
    env_logger::init();

    info!("Starting the gateway server on port 8000");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
