use config::Config;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[doc = "Application settings parsed from \"application-{env}.yaml\""]
#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub server:   ServerSettings,
    pub jwt:      JwtSettings,
    pub database: DatabaseSettings,
}

#[doc = "Server settings"]
#[derive(serde::Deserialize, Clone)]
pub struct ServerSettings {
    pub host:          String,
    pub port:          u16,
    pub cors_location: String,
    pub api_prefix:    String,
}

#[doc = "JWT settings"]
#[derive(serde::Deserialize, Clone)]
pub struct JwtSettings {
    pub secret:        Secret<String>,
    pub expires_in:    String,
    pub max_age:       i64,
    pub cookie_domain: String,
}

#[doc = "Settings for the database"]
#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username:      Secret<String>,
    pub password:      Secret<String>,
    pub port:          u16,
    pub host:          String,
    pub database_name: String,
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let settings: Config = Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn get_options(&self) -> PgConnectOptions {
        self.get_options_without_db().database(&self.database_name)
    }

    pub fn get_options_without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username.expose_secret())
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(PgSslMode::Prefer)
    }

    pub fn get_connection_string(&self) -> String {
        format!(
            "{}{}",
            self.get_connection_string_without_db(),
            self.database_name
        )
    }

    pub fn get_connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/",
            self.username.expose_secret(),
            self.password.expose_secret(),
            self.host,
            self.port
        )
    }
}

impl ServerSettings {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
