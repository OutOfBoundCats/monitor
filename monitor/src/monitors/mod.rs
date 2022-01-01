use chrono::prelude::*;

use std::{
    convert::TryInto,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use chrono::{DateTime, Duration, Utc};

use crate::{
    config::common::{Groups, Settings},
    monitors::{
        cpu::cpu_monitor, memory::memory_monitor, ping::ping_monitor, services::service_monitor,
        volume::volume_monitor,
    },
};

pub mod cpu;
pub mod memory;
pub mod ping;
pub mod services;
pub mod volume;
//use cpu::get_percentage_cpu_usage;

use crate::notifications::read_google_config::GoogleChatConfig;

#[tracing::instrument(skip(settings))]
pub fn monitor(settings: Settings) -> Vec<JoinHandle<()>> {
    tracing::info!("Started reading google chat config file");
    let google_chat_config = GoogleChatConfig::read_from_file();

    let google_chat_config = Arc::new(google_chat_config);

    let mut thread_handle = vec![];

    tracing::info!("Monitoring started");

    //iterate over all groups and start monitoring every item on different thread
    //1.cpu

    let l_google_chat_config = google_chat_config.clone();
    let l_settings = settings.clone();
    thread_handle.push(thread::spawn(move || {
        cpu_monitor(l_google_chat_config, l_settings);
    }));

    //2. volume moonitor
    let l_google_chat_config = google_chat_config.clone();
    let l_settings = settings.clone();

    //create different thread to monitor each mounting point mentioend in Item
    for item in settings.groups.volumes.items {
        let l_item = item;
        thread_handle.push(thread::spawn(move || {
            volume_monitor(l_google_chat_config, l_settings, l_item);
        }));
    }

    // 3. memory
    let l_google_chat_config = google_chat_config.clone();
    let l_settings = settings.clone();
    thread_handle.push(thread::spawn(move || {
        memory_monitor(l_google_chat_config, l_settings);
    }));

    // 4. pings
    let l_google_chat_config = google_chat_config.clone();
    let l_settings = settings.clone();
    for item in settings.groups.pings.items {
        let l_item = item;
        thread_handle.push(thread::spawn(move || {
            ping_monitor(l_google_chat_config, l_settings, l_item);
        }));
    }

    // 5. services
    let l_google_chat_config = google_chat_config.clone();
    let l_settings = settings.clone();

    for item in settings.groups.services.items {
        let l_item = item;
        thread_handle.push(thread::spawn(move || {
            service_monitor(l_google_chat_config, l_settings, l_item);
        }));
    }

    thread_handle
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
