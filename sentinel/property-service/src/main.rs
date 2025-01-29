use property_service::handlers;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/properties", web::post().to(handlers::create_property))
            .route("/properties", web::get().to(handlers::get_all_properties))
            .route("/properties/{id}", web::get().to(handlers::get_property))
            .route("/properties/{id}", web::put().to(handlers::update_property))
            .route("/properties/{id}", web::delete().to(handlers::delete_property))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
