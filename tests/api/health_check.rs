use crate::helpers::spawn_app;

#[tokio::test(flavor = "multi_thread")]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let health_url = format!("http://{}/health_check", app.address);

    let response = client
        .get(health_url)
        .send()
        .await
        .expect("Request failed.");

    assert!(response.status().is_success());
    assert_eq!("Ok", response.text_with_charset("utf-8").await.unwrap())
}