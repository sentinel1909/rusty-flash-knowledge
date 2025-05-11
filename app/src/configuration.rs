// app/src/configuration.rs

// dependencies
use pavex::blueprint::Blueprint;
use pavex::server::IncomingStream;
use pavex::t;
use pavex::time::SignedDuration;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode};

/// Refer to Pavex's [configuration guide](https://pavex.dev/docs/guide/configuration) for more details
/// on how to manage configuration values.
pub fn register(bp: &mut Blueprint) {
    bp.config("server", t!(self::ServerConfig));
    bp.config("database", t!(self::DatabaseConfig));
    bp.config("authorization", t!(self::AuthConfig));
    bp.config("templateconfig", t!(pavex_tera_template::TemplateConfig));
    bp.config(
        "staticserverconfig",
        t!(pavex_static_files::StaticServerConfig),
    );
}

// struct type to represent server configuration
#[derive(serde::Deserialize, Debug, Clone)]
/// Configuration for the HTTP server used to expose our API
/// to users.
pub struct ServerConfig {
    /// The port that the server must listen on.
    ///
    /// Set the `PX_SERVER__PORT` environment variable to override its value.
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub port: u16,
    /// The network interface that the server must be bound to.
    ///
    /// E.g. `0.0.0.0` for listening to incoming requests from
    /// all sources.
    ///
    /// Set the `PX_SERVER__IP` environment variable to override its value.
    pub ip: std::net::IpAddr,
    /// The timeout for graceful shutdown of the server.
    ///
    /// E.g. `1 minute` for a 1 minute timeout.
    ///
    /// Set the `PX_SERVER__GRACEFUL_SHUTDOWN_TIMEOUT` environment variable to override its value.
    #[serde(deserialize_with = "deserialize_shutdown")]
    pub graceful_shutdown_timeout: std::time::Duration,
}

// function to aid in shutdown configuration
fn deserialize_shutdown<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let duration = SignedDuration::deserialize(deserializer)?;
    if duration.is_negative() {
        Err(serde::de::Error::custom(
            "graceful shutdown timeout must be positive",
        ))
    } else {
        duration.try_into().map_err(serde::de::Error::custom)
    }
}

// methods for the server config struct type
impl ServerConfig {
    /// Bind a TCP listener according to the specified parameters.
    pub async fn listener(&self) -> Result<IncomingStream, std::io::Error> {
        let addr = std::net::SocketAddr::new(self.ip, self.port);
        IncomingStream::bind(addr).await
    }
}

// struct type to represent the database configuration
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

// methods for the database configuration type
impl DatabaseConfig {
    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.database_name);
        options
            .clone()
            .log_statements(tracing_log::log::LevelFilter::Trace);

        options
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .ssl_mode(ssl_mode)
    }

    pub async fn get_pool(&self) -> PgPool {
        PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(self.with_db())
    }
}

// struct type to represent the API key configuration value
#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    pub api_key: String,
}
