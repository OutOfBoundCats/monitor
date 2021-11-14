use std::sync::mpsc;
use std::time::Duration;
use std::{panic, thread};

mod config;
use crate::config::common::Settings;
use crate::config::tracing::*;

use tracing::subscriber::set_global_default;
use tracing::{info, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[actix_web::main]
async fn main() {
    //initialize thetracing crate
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // gives json output og log which we utilize to write to db
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);

    // specify how the log file should get created
    let file_appender = tracing_appender::rolling::never("application_log", "application.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    //make subscriber which  logs tracing events to console as well as file
    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt::Layer::default().with_writer(non_blocking))
        .with(JsonStorageLayer);

    set_global_default(subscriber).expect("Failed to set subscriber");
    //init_subscriber(subscriber);

    info!("Subsciber set");

    //new implementation of reading json config file
    let settings = Settings::from_setting();
    let i = match settings.groups[0].item_sleep {
        Some(value) => value,
        None => 1000,
    };
    println!("{}", &i);
}
