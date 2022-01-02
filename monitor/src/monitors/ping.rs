use std::{convert::TryInto, sync::Arc, thread};

use pinger::{ping, PingResult};

use crate::{
    config::common::{PingItems, Settings},
    notifications::read_google_config::GoogleChatConfig,
};

use super::thread_sleep;

pub fn pin_host(host: String) -> bool {
    let mut ping_response: bool = false;
    let stream = ping(host).expect("Error pinging");
    for message in stream {
        match message {
            PingResult::Pong(duration, _) => {
                println!("{:?}", duration);
                ping_response = true;
            }
            PingResult::Timeout(_) => {
                println!("Timeout!");
                ping_response = false;
            }
            PingResult::Unknown(line) => (),
        }
    }

    ping_response
}

//ping monitor
#[tracing::instrument(skip())]
pub fn ping_monitor(
    google_chat_config: Arc<GoogleChatConfig>,
    settings: Settings,
    item: PingItems,
) {
    tracing::info!("Started Ping Monitor");

    let inactive_days = settings.main.general.inactive_days;
    let inactive_times = settings.main.general.inactive_times;
    let mut notified: bool = false;
    let mut msg_index: i32 = 1;
    let mut notification_count = 0;
    let mut send_limit: i32;
    let mut severity = 2;

    match item.send_limit {
        Some(value) => {
            send_limit = value;
        }
        None => {
            send_limit = settings.main.notification.send_limit;
        }
    }

    loop {
        //sleep thread if current time falls between inactive time specified in json config
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili: i32;
        match item.item_sleep {
            Some(value) => {
                item_sleep_mili = value * 1000;
            }
            None => {
                tracing::error!("Error in getting the cpu group item_sleep time");
                item_sleep_mili = settings.main.notification.item_sleep * 1000;
            }
        }

        let mut l_first_wait;
        match item.first_wait {
            Some(value) => {
                l_first_wait = value;
            }
            None => {
                l_first_wait = settings.main.notification.first_wait;
            }
        };

        let mut l_wait_between;
        match item.wait_between {
            Some(value) => {
                l_wait_between = value;
            }
            None => {
                l_wait_between = settings.main.notification.wait_between;
            }
        };

        let mut priority;
        match item.priority {
            Some(value) => {
                priority = value;
            }
            None => {
                priority = settings.main.notification.priority;
            }
        };

        let ping_result = pin_host(item.target.clone());

        if ping_result && notified == true {
            notified = false;
            notification_count = 0;
            severity = 2;
            msg_index = 1; // select positive msg from array

            let message = get_message(msg_index, &settings.groups.pings.messages, &item.label);

            let l_msg = google_chat_config.build_msg(
                severity,
                &message,
                priority,
                &item.label,
                &item.target,
            );

            google_chat_config.send_chat_msg(l_msg);
            notification_count = 0;

            notified = false;

            thread::sleep(std::time::Duration::from_millis(
                (item_sleep_mili).try_into().unwrap(),
            ));
        } else if ping_result == false && notification_count <= send_limit {
            severity = 2;
            tracing::error!("");

            let message = get_message(msg_index, &settings.groups.pings.messages, &item.label);

            let l_msg = google_chat_config.build_msg(
                severity,
                &message,
                priority,
                &item.label,
                &item.target,
            );

            google_chat_config.send_chat_msg(l_msg);

            //for 1st msg wait for first wait
            if notified == false {
                thread::sleep(std::time::Duration::from_millis(
                    (l_first_wait * 1000).try_into().unwrap(),
                ));
            }

            //for subsequent messages wait for wait between
            if notified == true {
                thread::sleep(std::time::Duration::from_millis(
                    (l_wait_between * 1000).try_into().unwrap(),
                ));
            }

            //increase count and set nofified to true to keep track
            notification_count = notification_count + 1;
            notified = true;
        } else if ping_result == false && notification_count > send_limit {
            severity = 1;

            msg_index = 0;

            let message = get_message(msg_index, &settings.groups.pings.messages, &item.label);

            let l_msg = google_chat_config.build_msg(
                severity,
                &message,
                priority,
                &item.label,
                &item.target,
            );

            google_chat_config.send_chat_msg(l_msg);

            notification_count = 0;
            notified = false;
        }
    }
}

pub fn get_message(msg_index: i32, messages: &Vec<String>, label: &String) -> String {
    let l_msg_index: usize = msg_index.try_into().unwrap();
    let mut l_message = &messages[l_msg_index];
    let mut l_label = label;

    let mut l_message_1;

    l_message_1 = l_message.replacen("{{}}", l_label, 1);

    l_message_1
}
