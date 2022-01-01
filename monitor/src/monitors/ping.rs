use std::sync::Arc;

use pinger::{ping, PingResult};

use crate::{config::common::Settings, notifications::read_google_config::GoogleChatConfig};

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
pub fn ping_monitor(google_chat_config: Arc<GoogleChatConfig>, settings: Settings) {
    let google_chat_mutex = google_chat;

    let mut notified: bool = false;
    let mut notification_count = 0;

    loop {
        let severity = 2;
        tracing::info!("Ping monitor loop");
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
