use crate::helpers::{build_client, spawn_app};

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = build_client();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
