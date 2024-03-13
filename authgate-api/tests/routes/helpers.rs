use actix_web::dev::Server;
use authgate::config::settings::{get_settings, DatabaseSettings, Settings};
use authgate::startup::run;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub port:    u16,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let settings: Settings = {
        let mut c: Settings = get_settings().expect("Failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.database.port = 5432;
        c.database.host = "localhost".into();
        c.server.port = 0; // Random port for each test
        c.server.host = "0.0.0.0".into(); // Connect to localhost
        c
    };

    let database = configure_database(&settings.database).await;

    let listener = TcpListener::bind(format!("{}:{}", settings.server.host, settings.server.port))
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let _ = tokio::spawn(run_until_stopped(
        run(listener, database.clone(), settings.clone())
            .await
            .expect("Failed to startup"),
    ));

    TestApp {
        address: format!(
            "http://{}:{}{}",
            settings.server.host, port, settings.server.api_prefix
        ),
        port,
        db_pool: database,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create the database
    let mut connection: PgConnection =
        PgConnection::connect(&config.get_connection_string_without_db())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(
            r#"
            CREATE DATABASE "{}";
            "#,
            config.database_name
        ))
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.get_connection_string())
        .await
        .expect("Failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

async fn run_until_stopped(server: Server) -> Result<(), std::io::Error> {
    server.await
}
