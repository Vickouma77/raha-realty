use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use gateway::auth::AuthMiddleware;
use gateway::config::AppState;
use gateway::proxy::proxy;
use gateway::routes::{login, register};
use log::info;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    // Initialize application state
    let app_state = web::Data::new(AppState {
        client: Default::default(),
        services: Arc::new(Default::default()),
        auth_token: "my-secret-token".to_string(),
    });

    info!("Starting the gateway server on port 8000");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(AuthMiddleware::new(app_state.auth_token.clone()))
            .wrap(middleware::Logger::default())
            .route("/login", web::get().to(register))
            .route("/register", web::get().to(login))
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::Ok().finish() }),
            )
            .route(
                "/secure",
                web::get().to(|| async { HttpResponse::Ok().body("Secure data") }),
            )
            .route("/api/{service_name}/{tail:.*}", web::to(proxy))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
