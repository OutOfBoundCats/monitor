use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    convert::TryInto,
    ops::{Deref, DerefMut},
    os::unix::prelude::JoinHandleExt,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use chrono::{DateTime, Duration, Utc};

use crate::config::common::{Groups, Settings};

pub mod cpu;
pub mod disk;
pub mod memory;
pub mod ping;
pub mod services;
//use cpu::get_percentage_cpu_usage;

use crate::notifications::read_google_config::GoogleChatConfig;

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
    pub message: Vec<String>,
}
use actix_rt::{Arbiter, System};
#[tracing::instrument(skip(inactive_times, inactive_days, groups))]
pub fn monitor(
    settings: &Settings,
    groups: &Vec<Groups>,
    inactive_times: &Vec<(String, String)>,
    inactive_days: &Vec<String>,
) -> Vec<JoinHandle<()>> {
    let g_url = &settings.main.notification.url;

    let g_chat_info = GoogleChatConfig::read_from_file(g_url.clone());

    let g_chat_info = Arc::new(Mutex::new(g_chat_info));

    let mut thread_handle = vec![];

    tracing::info!("In Monitor method");

    for group in groups.iter() {
        let group_msg = group.messages.clone();
        for item in group.items.iter() {
            //each thread gets its own copy of data
            let local_item = LocalItems {
                name: item.name.clone(),
                label: item.label.clone(),
                target: item.target.clone(),
                priority: item.priority.unwrap().clone(),
                first_wait: item.first_wait.unwrap().clone(),
                wait_between: item.wait_between.unwrap().clone(),
                send_limit: item.send_limit.unwrap().clone(),
                item_sleep: item.item_sleep.unwrap().clone(),
                message: group_msg.clone(),
            };
            let local_inactive_days = inactive_days.clone();
            let local_inactive_times = inactive_times.clone();

            if item.name == "CPU" {
                let local_g_chat_info = g_chat_info.clone();
                thread_handle.push(thread::spawn(move || {
                    tracing::info!("Started CPU Monitor");

                    cpu_monitor(
                        local_g_chat_info,
                        local_item,
                        local_inactive_times,
                        local_inactive_days,
                    );
                }));
            } else if item.name == "DISK" {
                let local_g_chat_info = Arc::clone(&g_chat_info);
                thread_handle.push(thread::spawn(move || {
                    disk_monitor(
                        local_g_chat_info,
                        local_item,
                        local_inactive_times,
                        local_inactive_days,
                    )
                }));
            } else if item.name == "MEMORY" {
                let local_g_chat_info = Arc::clone(&g_chat_info);
                thread_handle.push(thread::spawn(move || {
                    memory_monitor(
                        local_g_chat_info,
                        local_item,
                        local_inactive_times,
                        local_inactive_days,
                    )
                }));
            } else if item.name == "PING" {
                let local_g_chat_info = g_chat_info.clone();
                let url = item.target.clone();
                thread_handle.push(thread::spawn(move || {
                    ping_monitor(
                        local_g_chat_info,
                        url,
                        local_item,
                        local_inactive_times,
                        local_inactive_days,
                    )
                }));
            } else if item.name == "SERVICE" {
                let local_g_chat_info = Arc::clone(&g_chat_info);
                thread_handle.push(thread::spawn(move || {
                    service_monitor(
                        local_g_chat_info,
                        local_item,
                        local_inactive_times,
                        local_inactive_days,
                    )
                }));
            } else {
                tracing::error!("item unspecified {}", &local_item.name);
            }
        }
    }

    thread_handle
}

//starts CPU monitoring
#[tracing::instrument(skip(item, inactive_times, inactive_days, google_chat))]
pub fn cpu_monitor(
    google_chat: Arc<Mutex<GoogleChatConfig>>,
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    let google_chat_mutex = google_chat.lock().unwrap();
    let mut notified: bool = false;
    let mut notification_count = 0;
    loop {
        let severity = 2;
        tracing::info!("CPU monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let cpu_usage = cpu::cpu_usage();
        tracing::info!("CPU uasge is {}", &cpu_usage);
        if cpu_usage > 90.0 && notification_count <= item.send_limit {
            //notify
            let message = google_chat_mutex.build_msg(
                &item,
                "ERROR",
                severity,
                "CPU usage more than 90%".to_string(),
            );

            let res = google_chat_mutex.send_chat_msg(message);

            notified = true;
            notification_count = notification_count + 1;
            if notification_count == 1 {
                thread::sleep(std::time::Duration::from_millis(
                    (item.first_wait * 1000).try_into().unwrap(),
                ));
            } else {
                thread::sleep(std::time::Duration::from_millis(
                    (item.wait_between * 1000).try_into().unwrap(),
                ));
            }

            tracing::error!("Cpu usage more than 90%");
        } else {
            notification_count = 0;
            notified = false;
        }

        if notified == false {
            thread::sleep(std::time::Duration::from_millis(
                item_sleep_mili.try_into().unwrap(),
            ));
        }
    }
}

//starts disk monitoring
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn disk_monitor(
    google_chat: Arc<Mutex<GoogleChatConfig>>,
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    let google_chat_mutex = google_chat.lock().unwrap();

    let mut notified: bool = false;
    let mut notification_count = 0;

    loop {
        let severity = 2;
        tracing::info!("Disk monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let disk_usage = disk::disk_capacity_usage();

        let mut messsage: String = "".to_string();

        for (disk_usage, mounted_on) in disk_usage {
            if disk_usage > 90 {
                tracing::error!(
                    "Mounted disk  on path {} is {} full",
                    &mounted_on,
                    &disk_usage
                );

                let temp = format!(
                    "Mounted disk  on path {} is <font color=\"#ff0000\"> {} %</font> full <br>",
                    &mounted_on, &disk_usage
                );

                messsage.push_str(&temp);
            }
        }

        let msg_len: usize = 0;
        if &messsage.len() != &msg_len && notification_count <= item.send_limit {
            let message = google_chat_mutex.build_msg(
                &item,
                "ERROR",
                severity,
                "CPU usage more than 90%".to_string(),
            );

            let res = google_chat_mutex.send_chat_msg(message);

            notified = true;
            notification_count = notification_count + 1;
            if notification_count == 1 {
                thread::sleep(std::time::Duration::from_millis(
                    (item.first_wait * 1000).try_into().unwrap(),
                ));
            } else {
                thread::sleep(std::time::Duration::from_millis(
                    (item.wait_between * 1000).try_into().unwrap(),
                ));
            }
        } else {
            notification_count = 0;
            notified = false;
        }

        if notified == false {
            thread::sleep(std::time::Duration::from_millis(
                item_sleep_mili.try_into().unwrap(),
            ));
        }
    }
}

//starts memory monitoring
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn memory_monitor(
    google_chat: Arc<Mutex<GoogleChatConfig>>,
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    let google_chat_mutex = google_chat.lock().unwrap();

    let mut notified: bool = false;
    let mut notification_count = 0;

    loop {
        let severity = 2;
        tracing::info!("Disk monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let memory_usage = memory::memory_usage();
        if memory_usage > 90 && notification_count <= item.send_limit {
            let message = google_chat_mutex.build_msg(
                &item,
                "ERROR",
                severity,
                format!("Memory usage very high at {} ", &memory_usage),
            );

            let res = google_chat_mutex.send_chat_msg(message);

            notified = true;
            notification_count = notification_count + 1;
            if notification_count == 1 {
                thread::sleep(std::time::Duration::from_millis(
                    (item.first_wait * 1000).try_into().unwrap(),
                ));
            } else {
                thread::sleep(std::time::Duration::from_millis(
                    (item.wait_between * 1000).try_into().unwrap(),
                ));
            }

            tracing::error!("Memory usage very high at {} ", &memory_usage);
        } else {
            notification_count = 0;
            notified = false;
        }

        if notified == false {
            thread::sleep(std::time::Duration::from_millis(
                item_sleep_mili.try_into().unwrap(),
            ));
        }
    }
}

//ping monitor
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn ping_monitor(
    google_chat: Arc<Mutex<GoogleChatConfig>>,
    url: String,
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    let google_chat_mutex = google_chat.lock().unwrap();

    let mut notified: bool = false;
    let mut notification_count = 0;

    loop {
        let severity = 2;
        tracing::info!("Disk monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let ping_respose = ping::pin_host(url.clone());
        if ping_respose == true {
            tracing::info!("{} responded succesfully", &url);
        } else if ping_respose == false && notification_count <= item.send_limit {
            let message = google_chat_mutex.build_msg(
                &item,
                "ERROR",
                severity,
                format!("{} not responding ", &url),
            );

            let res = google_chat_mutex.send_chat_msg(message);

            notified = true;
            notification_count = notification_count + 1;
            if notification_count == 1 {
                thread::sleep(std::time::Duration::from_millis(
                    (item.first_wait * 1000).try_into().unwrap(),
                ));
            } else {
                thread::sleep(std::time::Duration::from_millis(
                    (item.wait_between * 1000).try_into().unwrap(),
                ));
            }

            tracing::error!("{} not responding ", &url);
        } else {
            notification_count = 0;
            notified = false;
        }

        if notified == false {
            thread::sleep(std::time::Duration::from_millis(
                item_sleep_mili.try_into().unwrap(),
            ));
        }
    }
}

//service monitor
#[tracing::instrument(skip(item, inactive_times, inactive_days))]
pub fn service_monitor(
    google_chat: Arc<Mutex<GoogleChatConfig>>,
    item: LocalItems,
    inactive_times: Vec<(String, String)>,
    inactive_days: Vec<String>,
) {
    let google_chat_mutex = google_chat.lock().unwrap();

    let mut notified: bool = false;
    let mut notification_count = 0;

    loop {
        let severity = 2;
        tracing::info!("Service monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let (service_status, service_msg) = services::check_service();

        if service_status == 0 {
            tracing::info!("Service functioning properly");
            tracing::info!("{}", service_msg);
        } else if service_status == 1 && notification_count <= item.send_limit {
            let message = google_chat_mutex.build_msg(
                &item,
                "ERROR",
                severity,
                format!("{} not responding ", &item.name),
            );

            let res = google_chat_mutex.send_chat_msg(message);

            notified = true;
            notification_count = notification_count + 1;
            if notification_count == 1 {
                thread::sleep(std::time::Duration::from_millis(
                    (item.first_wait * 1000).try_into().unwrap(),
                ));
            } else {
                thread::sleep(std::time::Duration::from_millis(
                    (item.wait_between * 1000).try_into().unwrap(),
                ));
            }

            tracing::error!("Warning Service not functioning properly");
            tracing::error!("{}", service_msg);
        } else if notification_count <= item.send_limit {
            let message = google_chat_mutex.build_msg(
                &item,
                "ERROR",
                severity,
                format!("{} not functioning properly ", &item.name),
            );

            let res = google_chat_mutex.send_chat_msg(message);

            notified = true;
            notification_count = notification_count + 1;
            if notification_count == 1 {
                thread::sleep(std::time::Duration::from_millis(
                    (item.first_wait * 1000).try_into().unwrap(),
                ));
            } else {
                thread::sleep(std::time::Duration::from_millis(
                    (item.wait_between * 1000).try_into().unwrap(),
                ));
            }

            tracing::error!("Error Service not functioning");
            tracing::error!("{}", service_msg);
        } else {
            notification_count = 0;
            notified = false;
        }

        if notified == false {
            thread::sleep(std::time::Duration::from_millis(
                item_sleep_mili.try_into().unwrap(),
            ));
        }
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
