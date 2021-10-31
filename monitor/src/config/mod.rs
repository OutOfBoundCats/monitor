use color_eyre::Result;
mod common;

use config::*;
use eyre::WrapErr;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
extern crate serde;
extern crate serde_value;

use serde_value::Value;
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
    pub fn from_setting() {
        common::write_struct();

        let data = fs::read_to_string("read_setting.json").expect("Unable to read file");
        let mut serialised: common::Settings = serde_json::from_str(data.as_str()).unwrap();

        let f = serialised.groups.list[0].items[0].wait_between;
        // match serialised.groups.list[0].items[0].first_wait {
        //     Some(value) => {
        //         println!("macth value is {}", &value);
        //         value
        //     }
        //     None => 0,
        // };
        serialised.put_default_from_head();
        let item = &serialised.groups.list[0].items[0];
        let a: String = get_field_by_name(item, "priority");
        println!("get field by name returned {}", &a);
        println!("raed value is {:?}", &f);
    }
}

fn get_field_by_name<T, R>(data: T, field: &str) -> R
where
    T: Serialize,
    R: DeserializeOwned,
{
    let mut map = match serde_value::to_value(data) {
        Ok(Value::Map(map)) => map,
        _ => panic!("expected a struct"),
    };

    let key = Value::String(field.to_owned());
    let value = match map.remove(&key) {
        Some(value) => value,
        None => panic!("no such field"),
    };
    println!("value is {:?}", &value);

    match R::deserialize(value) {
        Ok(r) => r,
        Err(_) => panic!("wrong type?"),
    }
}
