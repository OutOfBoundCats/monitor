use std::sync::mpsc;
use std::time::Duration;
use std::{panic, thread};

mod config;
use crate::config::common::Settings;
use crate::config::tracing::*;
mod monitors;

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

    let file_appender = tracing_appender::rolling::never("application_log", "application.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    //layer to log to standard output
    let fmt_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(true)
        .compact()
        .pretty();

    //layer to log to file
    let file_layer = fmt::layer()
        .with_target(true) // don't include event targets when logging
        .with_level(true)
        .with_ansi(false)
        .compact()
        .pretty()
        .with_writer(non_blocking);

    //http layer  the gaurd has to be in main to work as per documentation
    let (non_blocking_http, _guard_http) = tracing_appender::non_blocking(HttpWriter);
    let formatting_layer = BunyanFormattingLayer::new("monitor".into(), non_blocking_http);
    // let http_layer = fmt::layer()
    //     .with_target(true) // don't include event targets when logging
    //     .with_level(true)
    //     .with_ansi(false)
    //     .compact()
    //     .with_writer(non_blocking_http);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(file_layer)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        //.with(http_layer)
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

    let mut children = vec![];

    monitors::monitor();

    let percentage = monitors::cpu::get_percentage_cpu_usage().await;

    tracing::info!("cpu usage is {}", percentage);

    for i in 0..5 {
        // Spin up another thread
        children.push(thread::spawn(move || {
            //println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
