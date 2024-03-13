use crate::domain::app::ApplicationStatus;
use crate::startup::AppState;
use actix_web::web::Data;
use actix_web::{get, post, HttpResponse};
use serde_json::json;

#[tracing::instrument(name = "Retrieving the state of the application", skip(state))]
#[get("/authgate/status")]
pub async fn authgate_status_handler(state: Data<AppState>) -> HttpResponse {
    match ApplicationStatus::get_application_status(&state.db).await {
        Ok(r) => HttpResponse::Ok().json(json!(r)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Initialize the application by creating the root user",
    skip(state)
)]
#[post("/authgate/initialize")]
pub async fn authgate_initialize_handler(state: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
