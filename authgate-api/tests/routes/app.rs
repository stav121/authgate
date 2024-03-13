use crate::helpers::spawn_app;
use actix_web::dev::Response;
use authgate::domain::app::ApplicationStatus;
use serde_json::json;

#[tokio::test]
async fn authgate_status_returns_200_ok() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/authgate/status", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let res_body = response
        .json::<ApplicationStatus>()
        .await
        .expect("Failed to parse json response");
    assert_eq!(res_body.is_initialized, Some(false));
}
