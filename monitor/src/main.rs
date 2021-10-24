use actix_web::dev::Server;
use actix_web::{rt::System, web, App, HttpResponse, HttpServer};
use std::sync::mpsc;
use std::thread;

#[actix_web::main]
async fn main() {
    // define channel to controll actix thread
    let (tx, rx) = mpsc::channel();
    println!("start");
    //spwan thread and start actix on that thread controll the execution using the channel
    thread::spawn(move || {
        //System is a runtime manager.
        let sys = System::new("http-server"); //Create new system.This method panics if it can not create tokio runtime

        let srv = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8080")?
        .shutdown_timeout(6000) // <- Set shutdown timeout to 60 seconds
        .run();

        let _ = tx.send(srv);
        
        sys.run()//This function will start tokio runtime and will finish once the System::stop() message get called. Function f get called within tokio runtime context.
    });

    let srv = rx.recv().unwrap();
    //start the server and await
    let mut i=1;
    loop{
        i=i+1;
        if (i==10000000){
            break;
        }
    }
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