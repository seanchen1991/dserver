use std::net::TcpListener;

const RANDOM_PORT: &str = "127.0.0.1:0";

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind(RANDOM_PORT).expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = dserver::run(listener).expect("Failed to bind to address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
