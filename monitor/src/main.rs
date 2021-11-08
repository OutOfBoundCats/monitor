use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{rt::System, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
mod controllers;
use controllers::app_config;
mod config;
use crate::config::common::Settings;

use crate::config::AppConfig;

use tracing::subscriber::set_global_default;

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[actix_web::main]
async fn main() {
    let settings = Settings::from_setting();
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);

    let file_appender = tracing_appender::rolling::never("application_log", "application.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt::Layer::default().with_writer(non_blocking))
        .with(JsonStorageLayer)
        //.with(formatting_layer)
        ;

    set_global_default(subscriber).expect("Failed to set subscriber");
    tracing::info!("hello");

    // define channel to controll actix thread
    let (tx, rx) = mpsc::channel();
    //read config from json
    let config = AppConfig::from_env().expect("Server configuration Errro");
    println!("{}", config.host);

    println!("config is {}", config.host);
    println!("start");
    //spwan thread and start actix on that thread controll the execution using the channel
    thread::spawn(move || {
        //System is a runtime manager.
        let sys = System::new("http-server"); //Create new system.This method panics if it can not create tokio runtime
        let srv = HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .configure(app_config)
                .route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8080")?
        .shutdown_timeout(6000) // <- Set shutdown timeout to 60 seconds
        .run();
        let _ = tx.send(srv);

        sys.run() //This function will start tokio runtime and will finish once the System::stop() message get called. Function f get called within tokio runtime context.
    });

    let srv = rx.recv().unwrap();

    thread::spawn(|| loop {
        println!("hi number from the spawned thread!");
        tracing::info!("hello");
        thread::sleep(Duration::from_millis(100));
    });

    srv.await;
    //start the server and await
    // let mut i=1;
    // loop{
    //     i=i+1;
    //     if (i==10000000){
    //         break;
    //     }
    // }

    // thread::spawn(move || {
    //    //notification
    // });

    // thread::spawn(move || {
    //     //monitoring
    //  });

    //Server::build();
    // pause accepting new connections
    println!("Pausing");
    //srv.pause().await;
    // resume accepting new connections
    println!("Resuming");
    //srv.resume().await;
    // stop server
    println!("Stopping");
    //srv.stop(true).await;
}
