use dserver::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

const RANDOM_PORT: &str = "127.0.0.1:0";
const HOST: &str = "http://127.0.0.1";

pub struct TestApp {
    pub address: String,
}

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind(RANDOM_PORT).expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("{}:{}", HOST, port);

    let _configuration = get_configuration().expect("Failed to read configuration");

    let server = run(listener).expect("Failed to bind to address");
    let _ = tokio::spawn(server);

    TestApp { address }
}
