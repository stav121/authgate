use crate::config::settings::{ServerSettings, Settings};
use crate::routes::actuator::health_check_handler;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, web::scope, web::ServiceConfig};
use actix_web::{App, HttpServer};
use reqwest::header;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct AppState {}

fn create_config(conf: &mut ServiceConfig, settings: &Settings) {
    conf.service(scope(&settings.server.api_prefix).service(health_check_handler));
}

pub async fn run(
    listener: TcpListener,
    pg_pool: PgPool,
    config: Settings,
) -> Result<Server, std::io::Error> {
    let server: Server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.server.cors_location)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState {}))
            .configure(|c| create_config(c, &config))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
