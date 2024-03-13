use actix_web::get;
use actix_web::HttpResponse;

/// API Resource: /actuator/health_check \[GET\]
///
/// Check the status of the application.
///
/// # Returns
/// * HTTP 200/OK if the application is up and running
#[tracing::instrument(name = "Checking application health")]
#[get("/actuator/health_check")]
async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}
