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
    let mut b = "{} hi there";
    let a = 5;
    //let xyz = format!(b, &a);

    use std::fmt::Write;

    let mut output = String::new();
    write!(&mut output, "{} {}", b, "world")
        .expect("Error occurred while trying to write in String");

    ///////////

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

    tracing::info!("Subsciber set");

    //new implementation of reading json config file
    let settings = Settings::from_setting();
    tracing::info!("Read configuration file");

    //start monitoring services and get the handle to all the thread started so we can join in main thread
    let child_threads = monitors::monitor(
        &settings.groups,
        &settings.main.general.inactive_times,
        &settings.main.general.inactive_days,
    );
    tracing::info!("Started monitoring threads");

    for child in child_threads {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
