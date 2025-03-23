use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;


pub fn run(listen: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
            )
    })
    .listen(listen)?
    .run();

    Ok(server)
}