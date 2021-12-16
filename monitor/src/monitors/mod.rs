use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::{
    convert::TryInto,
    os::unix::prelude::JoinHandleExt,
    thread::{self, JoinHandle},
};

use chrono::{DateTime, Duration, Utc};

use crate::config::common::Groups;

pub mod cpu;
pub mod disk;
pub mod memory;
pub mod ping;
pub mod services;
//use cpu::get_percentage_cpu_usage;

#[derive(Deserialize, Serialize, Debug)]
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
use actix_rt::{Arbiter, System};
#[tracing::instrument(skip(inactive_times, inactive_days, groups))]
pub fn monitor(
    groups: &Vec<Groups>,
    inactive_times: &Vec<(String, String)>,
    inactive_days: &Vec<String>,
) -> Vec<JoinHandle<()>> {
    let mut thread_handle = vec![];

    tracing::info!("In Monitor method");

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
                thread_handle.push(thread::spawn(move || {
                    tracing::info!("Started CPU Monitor");
                    cpu_monitor(local_item, local_inactive_times, local_inactive_days);
                }));
            } else if item.name == "DISK" {
                thread_handle.push(thread::spawn(move || {
                    disk_monitor(local_item, local_inactive_times, local_inactive_days)
                }));
            } else if item.name == "MEMORY" {
                thread_handle.push(thread::spawn(move || {
                    memory_monitor(local_item, local_inactive_times, local_inactive_days)
                }));
            } else if item.name == "PING" {
                let url = item.target.clone();
                thread_handle.push(thread::spawn(move || {
                    ping_monitor(url, local_item, local_inactive_times, local_inactive_days)
                }));
            } else if item.name == "SERVICE" {
                thread_handle.push(thread::spawn(move || {
                    service_monitor(local_item, local_inactive_times, local_inactive_days)
                }));
            } else {
                tracing::error!("item unspecified {}", &local_item.name);
            }
        }
    }

    thread_handle
}

//starts CPU monitoring
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn cpu_monitor(
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    loop {
        tracing::info!("CPU monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let cpu_usage = cpu::cpu_usage();
        tracing::info!("CPU uasge is {}", &cpu_usage);
        if cpu_usage > 90.0 {
            //notify

            tracing::info!("Cpu usage more than 90%");
        }

        thread::sleep(std::time::Duration::from_millis(
            item_sleep_mili.try_into().unwrap(),
        ));
    }
}

//starts disk monitoring
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn disk_monitor(
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    loop {
        tracing::info!("Disk monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let disk_usage = disk::disk_capacity_usage();

        for (disk_usage, mounted_on) in disk_usage {
            if disk_usage > 90 {
                tracing::info!(
                    "Mounted disk  on path {} is {} full",
                    &mounted_on,
                    &disk_usage
                );
            }
        }

        thread::sleep(std::time::Duration::from_millis(
            item_sleep_mili.try_into().unwrap(),
        ));
    }
}

//starts memory monitoring
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn memory_monitor(
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    loop {
        tracing::info!("Disk monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let memory_usage = memory::memory_usage();

        thread::sleep(std::time::Duration::from_millis(
            item_sleep_mili.try_into().unwrap(),
        ));
    }
}

//ping monitor
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn ping_monitor(
    url: String,
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    loop {
        tracing::info!("Disk monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let ping_respose = ping::pin_host(url.clone());
        if ping_respose == true {
            tracing::info!("{} responded succesfully", &url);
        } else {
            tracing::error!("{} not responding ", &url);
        }

        thread::sleep(std::time::Duration::from_millis(
            item_sleep_mili.try_into().unwrap(),
        ));
    }
}

//service monitor
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn service_monitor(
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    loop {
        tracing::info!("Service monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let (service_status, service_msg) = services::check_service();

        if service_status == 0 {
            tracing::info!("Service functioning properly");
            tracing::info!("{}", service_msg);
        } else if service_status == 1 {
            tracing::info!("Warning Service not functioning properly");
            tracing::info!("{}", service_msg);
        } else if service_status == 2 {
            tracing::info!("Error Service not functioning");
            tracing::info!("{}", service_msg);
        }

        thread::sleep(std::time::Duration::from_millis(
            item_sleep_mili.try_into().unwrap(),
        ));
    }
}

//function to sleep thread based on configuration inactive dates
#[tracing::instrument(skip(inactive_times, inactive_days))]
pub fn thread_sleep(inactive_times: &Vec<(String, String)>, inactive_days: &Vec<String>) {
    let local_time = Local::now().timestamp_millis();

    //if current date is marked inactive sllep untill next day
    for inactive_day in inactive_days {
        tracing::info!("inactive day is {}", &inactive_day);

        let inactive_date = DateTime::parse_from_str(inactive_day, "%Y-%m-%d %H:%M:%S %:z")
            .unwrap()
            .date();

        let inactive_local_date = Local::now().date();

        if inactive_local_date == inactive_date {
            let next_day = Local::now().checked_add_signed(Duration::days(1)).unwrap();
            let sleep_time = next_day.timestamp_millis() - Local::now().timestamp_millis();
            tracing::info!("Current date is marked inactive sleeping till next day");
            thread::sleep(std::time::Duration::from_millis(
                sleep_time.try_into().unwrap(),
            ));
        }
    }

    //if current time is in between inactivity tuple sleep till end time
    for inactive_time in inactive_times {
        let (start, end) = inactive_time;
        tracing::info!("inactivee time is from {} to {}", &start, &end);
        let start_dateTime = DateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S %:z")
            .unwrap()
            .timestamp_millis();
        let end_dateTime = DateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S %:z")
            .unwrap()
            .timestamp_millis();

        if local_time > start_dateTime && local_time < end_dateTime {
            let sleep_time = end_dateTime - local_time;
            tracing::info!(
                "System time lies between time specified in tuple sleeping for {} sec",
                &sleep_time
            );
            thread::sleep(std::time::Duration::from_millis(
                sleep_time.try_into().unwrap(),
            ));
        }
    }
}
