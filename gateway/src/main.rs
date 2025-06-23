use actix_web::{App, HttpServer, middleware, web};
use gateway::config::AppState;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    // Initialize application state
    let state = AppState::new();

    info!("Starting the gateway server on port 8000");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
