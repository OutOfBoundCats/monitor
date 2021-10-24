use color_eyre::Result;
use config::*;
use eyre::WrapErr;
use serde::Deserialize;
use std::path::Path;
use tracing::{info, instrument};

#[derive(Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: i32,
    pub key: String,
}

impl AppConfig {
    #[instrument]
    pub fn from_env() -> Result<AppConfig> {
        // tracing_subscriber::fmt()
        //     .with_env_filter(EnvFilter::from_default_env())
        //     .init();
        // info!("Loading configuration");

        let mut settings = Config::default();

        settings
            .merge(File::from(Path::new("configuration.json")))
            .unwrap();
        settings
            .try_into()
            .context("Loading configurations from environment")
    }
}
