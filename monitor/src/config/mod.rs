use color_eyre::Result;
pub mod common;

pub use config::*;
use eyre::WrapErr;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use tracing::{info, instrument};

#[derive(Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_resource")]
    pub host: String,
    pub port: i32,
    pub key: String,
}
fn default_resource() -> String {
    "localhost123".to_string()
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
        //if configuration missign default some predefined config
        settings
            .try_into()
            .context("Loading configurations from environment")
    }

    #[instrument]
    pub fn from_setting() -> common::Settings {
        common::write_struct();

        let data = fs::read_to_string("read_config.json").expect("Unable to read file");
        let mut serialised: common::Settings = serde_json::from_str(data.as_str()).unwrap();
        serialised.default_fill();
        let item_proprity = serialised.groups.list[0].items[0].priority;
        println!("new item priority {:?}", &item_proprity);
        serialised
    }
}
