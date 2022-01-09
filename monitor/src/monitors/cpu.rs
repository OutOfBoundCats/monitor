use std::{convert::TryInto, sync::Arc, thread};

use serde::{Deserialize, Serialize};
use systemstat::Platform;

use crate::{config::common::Settings, notifications::read_google_config::GoogleChatConfig};

use super::thread_sleep;

pub fn cpu_usage() -> f32 {
    let sys = systemstat::System::new();
    let _cpu_load = sys.cpu_load_aggregate().unwrap();
    let mut cpu_usage = 0.0;
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            std::thread::sleep(systemstat::Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            cpu_usage = 100.0 - cpu.idle * 100.0;
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }
    cpu_usage
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct localCpu {
    pub priority: Option<i32>,
    pub label: String,
    pub target: u8,
}

//starts CPU monitoring
#[tracing::instrument(skip(google_chat_config, settings))]
pub fn cpu_monitor(google_chat_config: Arc<GoogleChatConfig>, settings: Settings) {
    tracing::info!("Started CPU Monitor");

    let inactive_days = settings.main.general.inactive_days.clone();
    let inactive_times = settings.main.general.inactive_times.clone();

    //get all the active items only

    let mut vec_local_cpu: Vec<localCpu> = vec![];

    for item in &settings.groups.cpu.items {
        //convert target to u8
        let target_int = item.target.clone().parse::<u8>().unwrap();

        //if item is enabled then only monitor
        if item.enabled == true {
            let local_cpu = localCpu {
                priority: item.priority.clone(),
                label: item.label.clone(),
                target: target_int,
            };
            vec_local_cpu.push(local_cpu);
        }
    }

    //sort the vec in ascending order
    vec_local_cpu.sort_by_key(|x| x.target);

    //check if there are infact active element to monitor
    if vec_local_cpu.len() == 0 {
        tracing::info!("There are no active Cpu items to monitor")
    } else {
        //if there are items to monitor then continue monitoring
        let mut severity = 2;
        let send_limit: i32;
        let mut notification_count = 0;
        let mut notified: bool = false;
        let mut msg_index: i32;
        let mut item_sleep_mili: i32;
        let mut priority;
        let mut l_first_wait;
        let mut l_wait_between;

        match settings.groups.cpu.send_limit {
            Some(value) => {
                send_limit = value;
            }
            None => {
                send_limit = settings.main.notification.send_limit;
            }
        }

        match settings.groups.cpu.item_sleep {
            Some(value) => {
                item_sleep_mili = value * 1000;
            }
            None => {
                tracing::error!("Error in getting the cpu group item_sleep time");
                item_sleep_mili = settings.main.notification.item_sleep * 1000;
            }
        }

        match settings.groups.cpu.first_wait {
            Some(value) => {
                l_first_wait = value;
            }
            None => {
                l_first_wait = settings.main.notification.first_wait;
            }
        };

        match settings.groups.cpu.wait_between {
            Some(value) => {
                l_wait_between = value;
            }
            None => {
                l_wait_between = settings.main.notification.wait_between;
            }
        };

        match settings.groups.cpu.priority {
            Some(value) => {
                priority = value * 1000;
            }
            None => {
                tracing::error!("Error in getting the cpu group item_sleep time");
                priority = settings.main.notification.priority * 1000;
            }
        }

        loop {
            //sleep thread if current time falls between inactive time specified in json config
            thread_sleep(&inactive_times, &inactive_days);

            let cpu_usage = cpu_usage();

            for local_item in &vec_local_cpu {
                //check if cpu usage is more than target if yes then notify and skip checking next elements
                if cpu_usage > local_item.target.into() && send_limit < notification_count {
                    msg_index = 0; //select negative msg from array
                    severity = 2; //inform employees

                    let message =
                        get_message(msg_index, &settings.groups.cpu.messages, &local_item.label);

                    let l_msg = google_chat_config.build_msg(
                        severity,
                        &message,
                        priority,
                        &local_item.label,
                        &format!("{}", &local_item.target),
                    );

                    google_chat_config.send_chat_msg(l_msg);

                    notified = true;
                    notification_count = notification_count + 1;

                    continue;
                } else if cpu_usage > local_item.target.into() && send_limit > notification_count {
                    //if notification count is more than send limit and still issue exist notify management

                    msg_index = 0; //select negative msg from array
                    severity = 1; //inform employees

                    let message =
                        get_message(msg_index, &settings.groups.cpu.messages, &local_item.label);

                    let l_msg = google_chat_config.build_msg(
                        severity,
                        &message,
                        priority,
                        &local_item.label,
                        &format!("{}", &local_item.target),
                    );

                    google_chat_config.send_chat_msg(l_msg);

                    notified = true;
                    notification_count = 0;

                    continue;
                }
            }

            //check if current cpu usage is less than lowest target specified if yes send healthy message
            if cpu_usage < vec_local_cpu[0].target.into() && notified == true {
                msg_index = 1; //select positive msg from array
                severity = 1; //inform employees

                let message = get_message(
                    msg_index,
                    &settings.groups.cpu.messages,
                    &vec_local_cpu[0].label,
                );

                let l_msg = google_chat_config.build_msg(
                    severity,
                    &message,
                    priority,
                    &vec_local_cpu[0].label,
                    &format!("{}", &vec_local_cpu[0].target),
                );

                google_chat_config.send_chat_msg(l_msg);

                notified = false;
                notification_count = 0;
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
}

pub fn get_message(msg_index: i32, messages: &Vec<String>, label: &String) -> String {
    let l_msg_index: usize = msg_index.try_into().unwrap();
    let mut l_message = &messages[l_msg_index];
    let mut l_label = label;

    let mut l_message_1;

    l_message_1 = l_message.replacen("{{}}", l_label, 1);

    l_message_1
}
