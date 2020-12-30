use dserver::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;

    Ok(())
}
