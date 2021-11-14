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
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("monitor".into(), std::io::stdout);

    let file_appender = tracing_appender::rolling::never("application_log", "application.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let fmt_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(true)
        .compact()
        .pretty();

    let file_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(false)
        .compact()
        .with_writer(non_blocking);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        //.with(tracing_subscriber::fmt::layer())
        //.with(fmt::Layer::default().with_writer(non_blocking))
        .with(file_layer)
        //.with(JsonStorageLayer)
        //.with(formatting_layer)
        ;
    //let subscriber = get_subcriber();

    set_global_default(subscriber).expect("Failed to set subscriber");

    tracing::info!("hello");

    tracing::error!("Subsciber set");

    //new implementation of reading json config file
    let settings = Settings::from_setting();
    //println!("{:?}", &settings);

    let i = settings.groups[0].items[0].item_sleep;
    //println!("i is {:?}", &i);
}
