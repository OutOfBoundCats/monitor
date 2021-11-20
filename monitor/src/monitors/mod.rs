use std::{
    os::unix::prelude::JoinHandleExt,
    thread::{self, JoinHandle},
};

use crate::config::common::Groups;

pub mod cpu;

pub struct LocalItems {
    pub name: String,
    pub label: String,
    pub target: String,
    pub priority: i32,
    pub first_wait: i32,
    pub wait_between: i32,
    pub send_limit: i32,
    pub item_sleep: i32,
}

pub fn monitor(
    groups: &Vec<Groups>,
    inactive_times: &Vec<(String, String)>,
    inactive_days: &Vec<String>,
) -> Vec<JoinHandle<()>> {
    let mut children = vec![];

    for group in groups.iter() {
        for item in group.items.iter() {
            //each thread gets its own copy of data
            let local_item = LocalItems {
                name: item.name.clone(),
                label: item.name.clone(),
                target: item.name.clone(),
                priority: item.priority.unwrap().clone(),
                first_wait: item.priority.unwrap().clone(),
                wait_between: item.priority.unwrap().clone(),
                send_limit: item.priority.unwrap().clone(),
                item_sleep: item.priority.unwrap().clone(),
            };
            let local_inactive_days = inactive_days.clone();
            let local_inactive_times = inactive_times.clone();

            if item.name == "CPU" {
                children.push(thread::spawn(move || {
                    cpu_monitor(local_item, local_inactive_times, local_inactive_days)
                }));
            } else if item.name == "DISK" {
                children.push(thread::spawn(move || {
                    disk_monitor(local_item, local_inactive_times, local_inactive_days)
                }));
            } else {
                tracing::error!("item unspecified {}", &local_item.name);
            }
        }
    }

    for i in 0..5 {
        // Spin up another thread
        children.push(thread::spawn(move || {}));
    }

    children
}

pub fn cpu_monitor(
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
}
pub fn disk_monitor(
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
}
