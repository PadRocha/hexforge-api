use std::process::ExitCode;

use apihules::infra::{
    app::create_app,
    config::{AppConfig, load_environment},
    setup::{init_app_state, init_tracing},
    startup_error::StartupError,
};
use axum::serve;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

async fn run() -> Result<(), StartupError> {
    load_environment()?;
    init_tracing()?;

    let config = Arc::new(AppConfig::from_env()?);
    let app_state = init_app_state(config.clone()).await?;
    let app = create_app(app_state)?;

    let listener = TcpListener::bind(&config.server_address)
        .await
        .map_err(|source| StartupError::BindServer {
            address: config.server_address.clone(),
            source,
        })?;

    let local_addr = listener
        .local_addr()
        .map_err(|source| StartupError::LocalAddress { source })?;

    info!("Backend listening at {}", local_addr);

    serve(listener, app)
        .await
        .map_err(|source| StartupError::ServeHttp { source })?;

    Ok(())
}
