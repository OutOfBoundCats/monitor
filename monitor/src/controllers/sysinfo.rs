
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::{web::ServiceConfig, HttpRequest};

use std::collections::HashMap;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};


#[get("/health")]
pub async fn health() -> String  {
    "All Gooing Good".to_owned()
}
