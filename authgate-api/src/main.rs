use authgate::config::logging::{create_logging_subscriber, init_sub};
use authgate::config::settings::{get_settings, Settings};
use authgate::startup::run;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Setup logging
    let subscriber = create_logging_subscriber("authgate".into(), "info".into());
    init_sub(subscriber);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Load the application settings
    let settings: Settings = get_settings().expect("Failed to read application settings");

    // Setup the TCP Listener
    let listener: TcpListener =
        TcpListener::bind(settings.server.get_addr()).expect("Failed to bind address");

    // Connect to the database
    let pg_pool: PgPool = PgPoolOptions::new().connect_lazy_with(settings.database.get_options());

    run(listener, pg_pool, settings.clone()).await?.await
}
