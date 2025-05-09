// shuttle/src/main.rs

// dependencies
use pavex::config::ConfigLoader;
use pavex::server::Server;
use server::configuration::Profile::{Dev, Prod};
use server_sdk::{ApplicationConfig, ApplicationState};
use shuttle_runtime::{CustomError, SecretStore};

// module dependencies
mod shuttle_pavex;

#[shuttle_runtime::main]
async fn pavex(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_pavex::ShuttlePavex {
    let profile = secrets.get("PX_PROFILE").unwrap_or("dev".to_string());

    let app_profile = match profile.as_str() {
        "dev" => Dev,
        "prod"=> Prod,
        _ => panic!("Unable to set the application profile.") 
    };

    tracing::info!("Application profile (set from Secrets): {:?}", app_profile);
    
    let app_config: ApplicationConfig = ConfigLoader::new().profile(app_profile).load().map_err(|err| {
        let error_msg = format!("Unable to load the application profile: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    let app_state = ApplicationState::new(app_config).await.map_err(|err| {
        let error_msg = format!("Unable to build the application state: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    let app_server = Server::new();

    let shuttle_px = shuttle_pavex::PavexService(app_server, app_state);

    Ok(shuttle_px)
}
