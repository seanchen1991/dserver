const HEALTH_CHECK_ENDPOINT: &'static str = "http://127.0.0.1:8000/health_check";

#[actix_rt::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(HEALTH_CHECK_ENDPOINT)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = dserver::run().expect("Failed to bind to address");
    let _ = tokio::spawn(server);
}
