use std::sync::mpsc;
use std::time::Duration;
use std::{panic, thread};

mod config;
use crate::config::common::Settings;
use crate::config::tracing::*;

use env_logger::Env;
use tracing::subscriber::set_global_default;
use tracing::{info, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[actix_web::main]
async fn main() {
    let subscriber = get_subcriber();

    set_global_default(subscriber).expect("Failed to set subscriber");

    tracing::info!("hello");

    tracing::error!("Subsciber set");

    //new implementation of reading json config file
    let settings = Settings::from_setting();
    //println!("{:?}", &settings);

    let i = settings.groups[0].items[0].item_sleep;
    //println!("i is {:?}", &i);
}
