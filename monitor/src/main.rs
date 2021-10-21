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
        let sys = System::new("http-server");

        let srv = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8080")?
        .shutdown_timeout(6000) // <- Set shutdown timeout to 60 seconds
        .run();

        let _ = tx.send(srv);
        sys.run()
    });

    let srv = rx.recv().unwrap();
    //start the server and await
    srv.await;
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