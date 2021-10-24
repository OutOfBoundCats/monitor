use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

mod sysinfo;

pub fn app_config(config: &mut ServiceConfig) {
    config.service(sysinfo::health);

}

