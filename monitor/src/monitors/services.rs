use std::{convert::TryInto, sync::Arc, thread};

use run_script::ScriptOptions;

use crate::{
    config::common::{ServiceItems, Settings},
    monitors::thread_sleep,
    notifications::read_google_config::GoogleChatConfig,
};

pub fn check_service(command: String, expected_output: String) -> bool {
    let mut return_msg: String = "".to_string();
    let mut return_int: i32 = -1;

    let options = ScriptOptions::new();

    let args = vec![];
    let (code, output, error) =
        run_script::run(format!("{}", &command).as_str(), &args, &options).unwrap();

    if expected_output == output {
        return true;
    } else {
        return false;
    }
}

//service monitor
#[tracing::instrument(skip(google_chat_config, settings, item))]
pub fn service_monitor(
    google_chat_config: Arc<GoogleChatConfig>,
    settings: Settings,
    item: ServiceItems,
) {
    tracing::info!("Started Services Monitor for {}", &item.label);

    let inactive_days = settings.main.general.inactive_days;
    let inactive_times = settings.main.general.inactive_times;
    let mut notified: bool = false;
    let mut msg_index: i32;
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

    let item_sleep_mili: i32;
    match item.item_sleep {
        Some(value) => {
            item_sleep_mili = value * 1000;
        }
        None => {
            item_sleep_mili = settings.main.notification.item_sleep * 1000;
        }
    }

    let l_first_wait;
    match item.first_wait {
        Some(value) => {
            l_first_wait = value * 1000;
        }
        None => {
            l_first_wait = settings.main.notification.first_wait * 1000;
        }
    };

    let l_wait_between;
    match item.wait_between {
        Some(value) => {
            l_wait_between = value * 1000;
        }
        None => {
            l_wait_between = settings.main.notification.wait_between * 1000;
        }
    };

    let priority;
    match item.priority {
        Some(value) => {
            priority = value;
        }
        None => {
            priority = settings.main.notification.priority;
        }
    };

    loop {
        //sleep thread if current time falls between inactive time specified in json config
        thread_sleep(&inactive_times, &inactive_days);

        let service_status = check_service(item.command.clone(), item.output.clone());

        if service_status && notified == true {
            notified = false;
            notification_count = 0;
            severity = 2;
            msg_index = 1; // select positive msg from array

            tracing::info!(
                "Service Monitor detected issue for {} has been resolved",
                &item.label
            );

            let message = get_message(msg_index, &settings.groups.services.messages, &item.label);

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
        } else if service_status == false && notification_count <= send_limit {
            severity = 2;
            msg_index = 0;
            tracing::error!("Service Monitor detected issue for {}", &item.label);

            let message = get_message(msg_index, &settings.groups.services.messages, &item.label);

            let l_msg = google_chat_config.build_msg(
                severity,
                &message,
                priority,
                &item.label,
                &item.target,
            );

            google_chat_config.send_chat_msg(l_msg);

            //increase count and set nofified to true to keep track
            notification_count = notification_count + 1;
            notified = true;
        } else if service_status == false && notification_count > send_limit {
            severity = 1;
            msg_index = 0;

            let message = get_message(msg_index, &settings.groups.services.messages, &item.label);

            let l_msg = google_chat_config.build_msg(
                severity,
                &message,
                priority,
                &item.label,
                &item.target,
            );

            google_chat_config.send_chat_msg(l_msg);

            notification_count = notification_count + 1;
            notified = true;
        }

        //if there was no earlier notification sent then sleep thread for  item_sleep duration as per json
        if notified == false {
            thread::sleep(std::time::Duration::from_millis(
                (item_sleep_mili).try_into().unwrap(),
            ));
        }

        // if notification sent if 1st then sleep for 1st wait else wait for wait_between as per json
        if notified == true && notification_count == 1 {
            thread::sleep(std::time::Duration::from_millis(
                (l_first_wait).try_into().unwrap(),
            ));
        } else if notified == true && notification_count != 1 {
            thread::sleep(std::time::Duration::from_millis(
                (l_wait_between).try_into().unwrap(),
            ));
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
