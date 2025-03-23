use gateway::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = format!("127.0.0.1:8000");

    let listener = TcpListener::bind(address).expect("Failed to bind to port 8000");

    run(listener)?.await
}
