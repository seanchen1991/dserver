use dserver::run;
use std::net::TcpListener;

const PORT: &str = "127.0.0.1:0";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(PORT).expect("Failed to bind to port 8000");
    run(listener)?.await
}
