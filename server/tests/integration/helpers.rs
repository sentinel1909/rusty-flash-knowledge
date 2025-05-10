// server/tests/api/helpers.rs

// dependencies
use app::{UpdatedFlashCard, models::NewFlashCard};
use pavex::{
    config::ConfigLoader,
    http::{HeaderMap, HeaderValue},
    server::Server,
};
use reqwest::header::{AUTHORIZATION, HOST};
use server::configuration::Profile;
use server_sdk::{ApplicationConfig, ApplicationState, run};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::borrow::Cow;
use std::path::PathBuf;
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
    pub api_key: String,
}

impl TestApi {
    pub async fn spawn() -> Self {
        Self::init_telemetry();
        let mut config = Self::get_config();
        config.database.database_name = Uuid::new_v4().to_string();
        config.templateconfig.dir = Cow::Owned("../templates".to_string());
        config.staticserverconfig.root_dir = PathBuf::from("../static");
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
        let api_key = config.authorization.api_key.clone();

        let application_state = ApplicationState::new(config)
            .await
            .expect("Failed to build the application state");

        tokio::spawn(async move { run(server_builder, application_state).await });

        TestApi {
            api_address,
            api_client,
            api_db_pool,
            api_key,
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
    async fn set_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth = format!("Bearer {}", self.api_key);
        headers.insert(
            HOST,
            HeaderValue::from_static("api.rusty-flash-knowledge.net"),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth).expect("Invalid API key"),
        );

        headers
    }

    pub async fn post_no_api_key(&self) -> reqwest::Response {
        self.api_client
            .post(format!("{}/v1/flashcards", &self.api_address))
            .header(HOST, "api.rusty-flash-knowledge.net")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_invalid_api_key(&self) -> reqwest::Response {
        let mut headers = HeaderMap::new();
        headers.insert(
            HOST,
            HeaderValue::from_static("api.rusty-flash-knowledge.net"),
        );
        headers.insert(AUTHORIZATION, HeaderValue::from_static("the-wrong-api-key"));
        self.api_client
            .post(format!("{}/v1/flashcards", &self.api_address))
            .headers(headers)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_flashcards(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/flashcards", &self.api_address))
            .header(HOST, "rusty-flash-knowledge.net")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_flashcards_by_topic(&self, query: Option<&str>) -> reqwest::Response {
        let url = match query {
            Some(topic) => format!("{}/flashcards?topic={}", &self.api_address, topic),
            None => format!("{}/flashcards", &self.api_address),
        };
        self.api_client
            .get(url)
            .header(HOST, "rusty-flash-knowledge.net")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_flashcards_by_tag(&self, query: Option<&str>) -> reqwest::Response {
        let url = match query {
            Some(tag) => format!("{}/flashcards?tag={}", &self.api_address, tag),
            None => format!("{}/flashcards", &self.api_address),
        };
        self.api_client
            .get(url)
            .header(HOST, "rusty-flash-knowledge.net")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_all_tags(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/flashcards/tags", &self.api_address))
            .header(HOST, "rusty-flash-knowledge.net")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_all_topics(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/flashcards/topics", &self.api_address))
            .header(HOST, "rusty-flash-knowledge.net")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_flashcard(&self, id: String) -> reqwest::Response {
        self.api_client
            .get(format!("{}/flashcards/{}", &self.api_address, id))
            .header(HOST, "rusty-flash-knowledge.net")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn create_flashcard(&self, payload: &NewFlashCard) -> reqwest::Response {
        self.api_client
            .post(format!("{}/v1/flashcards", &self.api_address))
            .headers(self.set_headers().await)
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_flashcard(&self, id: String) -> reqwest::Response {
        self.api_client
            .delete(format!("{}/v1/flashcards/{}", &self.api_address, id))
            .headers(self.set_headers().await)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn update_flashcard(
        &self,
        payload: &UpdatedFlashCard,
        id: String,
    ) -> reqwest::Response {
        self.api_client
            .put(format!("{}/v1/flashcards/{}", &self.api_address, id))
            .headers(self.set_headers().await)
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
