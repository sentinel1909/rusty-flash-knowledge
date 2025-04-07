use app::models::NewFlashCard;
use pavex::{config::ConfigLoader, http::HeaderValue, server::Server};
use server::configuration::Profile;
use server_sdk::{ApplicationConfig, ApplicationState, run};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::Once;
use tracing::subscriber::set_global_default;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

// helper function to configure an independent database for each test, so that teste are isolated
async fn configure_database(config: &ApplicationConfig) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.database.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("../migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

pub struct TestApi {
    pub api_address: String,
    pub api_client: reqwest::Client,
    pub api_db_pool: PgPool,
}

impl TestApi {
    pub async fn spawn() -> Self {
        Self::init_telemetry();
        let mut config = Self::get_config();
        config.database.database_name = Uuid::new_v4().to_string();
        configure_database(&config).await;
        let tcp_listener = config
            .server
            .listener()
            .await
            .expect("Failed to bind the server TCP listener");
        let address = tcp_listener
            .local_addr()
            .expect("The server TCP listener doesn't have a local socket address");
        let server_builder = Server::new().listen(tcp_listener);
        let api_address = format!("http://{}:{}", config.server.ip, address.port());
        let api_client = reqwest::Client::new();
        let api_db_pool = config.database.get_pool().await;

        let application_state = ApplicationState::new(config)
            .await
            .expect("Failed to build the application state");

        tokio::spawn(async move { run(server_builder, application_state).await });

        TestApi {
            api_address,
            api_client,
            api_db_pool,
        }
    }

    /// Load the dev configuration and tweak it to ensure that tests are
    /// properly isolated from each other.
    fn get_config() -> ApplicationConfig {
        let mut config: ApplicationConfig = ConfigLoader::new()
            .profile(Profile::Dev)
            .load()
            .expect("Failed to load test configuration");
        // We use port `0` to get the operating system to assign us a random port.
        // This lets us run tests in parallel without running into "port X is already in use"
        // errors.
        config.server.port = 0;
        config
    }

    fn init_telemetry() {
        // Initialize the telemetry setup at most once.
        static INIT_TELEMETRY: Once = Once::new();
        INIT_TELEMETRY.call_once(|| {
            // Only enable the telemetry if the `TEST_LOG` environment variable is set.
            if std::env::var("TEST_LOG").is_ok() {
                let subscriber = tracing_subscriber::fmt::Subscriber::builder()
                    .with_env_filter(
                        EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")),
                    )
                    .finish();
                // We don't redirect panic messages to the `tracing` subsystem because
                // we want to see them in the test output.
                set_global_default(subscriber).expect("Failed to set a `tracing` global subscriber")
            }
        });
    }
}

/// Convenient methods for calling the API under test.
impl TestApi {
    pub async fn get_ping(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/v1/ping", &self.api_address))
            .header(
                reqwest::header::HOST,
                HeaderValue::from_static("api.rusty-flash-knowledge.net"),
            )
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_flashcards(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/v1/flashcards", &self.api_address))
            .header(
                reqwest::header::HOST,
                HeaderValue::from_static("api.rusty-flash-knowledge.net"),
            )
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn create_flashcard(&self, payload: &NewFlashCard) -> reqwest::Response {
        self.api_client
            .post(format!("{}/v1/flashcards", &self.api_address))
            .header(
                reqwest::header::HOST,
                HeaderValue::from_static("api.rusty-flash-knowledge.net"),
            )
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_flashcard(&self, id: String) -> reqwest::Response {
        self.api_client
            .delete(format!("{}/v1/flashcards/{}", &self.api_address, id))
            .header(
                reqwest::header::HOST,
                HeaderValue::from_static("api.rusty-flash-knowledge.net"),
            )
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
