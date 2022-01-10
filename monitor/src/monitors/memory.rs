use std::{convert::TryInto, sync::Arc, thread};

use serde::{Deserialize, Serialize};
use systemstat::Platform;

use crate::{
    config::common::Settings, monitors::thread_sleep,
    notifications::read_google_config::GoogleChatConfig,
};

pub fn memory_usage() -> (u64, u64, bool) {
    let sys = systemstat::System::new();
    let mut total_memory = 0;
    let mut used_memory = 0;
    let mut err: bool = false;
    match sys.memory() {
        Ok(mem) => {
            // println!(
            //     "\nMemory: {} used / {} ({} bytes) total ({:?})",
            //     saturating_sub_bytes(mem.total, mem.free),
            //     mem.total,
            //     mem.total.as_u64(),
            //     mem.platform_memory
            // );
            total_memory = mem.total.as_u64();
            used_memory = mem.total.as_u64() - mem.free.as_u64();
        }
        Err(x) => {
            err = true;
        }
    }
    // let percentage_used = (used_memory / total_memory) * 100;
    (used_memory, total_memory, err)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct localMemory {
    pub priority: Option<i32>,
    pub label: String,
    pub target: u8,
}

//starts memory monitoring
#[tracing::instrument(skip(google_chat_config, settings))]
pub fn memory_monitor(google_chat_config: Arc<GoogleChatConfig>, settings: Settings) {
    tracing::info!("Started Memory Monitor");

    let inactive_days = settings.main.general.inactive_days.clone();
    let inactive_times = settings.main.general.inactive_times.clone();

    //get all the active items only

    let mut vec_local_memory: Vec<localMemory> = vec![];

    for item in &settings.groups.memory.items {
        //convert target to u8
        let target_int = item.target.clone().parse::<u8>().unwrap();

        //if item is enabled then only monitor
        if item.enabled == true {
            let local_memory = localMemory {
                priority: item.priority.clone(),
                label: item.label.clone(),
                target: target_int,
            };
            vec_local_memory.push(local_memory);
        }
    }

    //sort the vec in ascending order
    vec_local_memory.sort_by_key(|x| x.target);

    //check if there are infact active element to monitor
    if vec_local_memory.len() == 0 {
        tracing::info!("There are no active Memory items to monitor")
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

        match settings.groups.memory.send_limit {
            Some(value) => {
                send_limit = value;
            }
            None => {
                send_limit = settings.main.notification.send_limit;
            }
        }

        match settings.groups.memory.item_sleep {
            Some(value) => {
                item_sleep_mili = value * 1000;
            }
            None => {
                tracing::error!("Error in getting the memory group item_sleep time");
                item_sleep_mili = settings.main.notification.item_sleep * 1000;
            }
        }

        match settings.groups.memory.first_wait {
            Some(value) => {
                l_first_wait = value;
            }
            None => {
                l_first_wait = settings.main.notification.first_wait;
            }
        };

        match settings.groups.memory.wait_between {
            Some(value) => {
                l_wait_between = value;
            }
            None => {
                l_wait_between = settings.main.notification.wait_between;
            }
        };

        match settings.groups.memory.priority {
            Some(value) => {
                priority = value;
            }
            None => {
                tracing::error!("Error in getting the memory group item_sleep time");
                priority = settings.main.notification.priority;
            }
        }

        let mut max_item_crossed: localMemory = localMemory {
            priority: Some(0),
            label: "0".to_string(),
            target: 0,
        };

        loop {
            //sleep thread if current time falls between inactive time specified in json config
            thread_sleep(&inactive_times, &inactive_days);

            let (used_memory, total_memory, err) = memory_usage();

            let memory_used_percentage = used_memory / total_memory * 100;

            let mut max_target_crossed: i32 = 0;

            //find maximum threashold crossed
            for local_item in &vec_local_memory {
                if memory_used_percentage > local_item.target.into() {
                    max_item_crossed = local_item.clone();
                    max_target_crossed = local_item.target.into();
                }
            }

            if max_target_crossed == 0 && notified == true {
                msg_index = 1; //select positive msg from array
                severity = 2; //inform employees

                let message = get_message(
                    msg_index,
                    &settings.groups.memory.messages,
                    &vec_local_memory[0].label,
                );

                let l_msg = google_chat_config.build_msg(
                    severity,
                    &message,
                    priority,
                    &vec_local_memory[0].label,
                    &format!("{}", &vec_local_memory[0].target),
                );

                google_chat_config.send_chat_msg(l_msg);

                notified = false;
                notification_count = 0;
            } else if max_target_crossed != 0 && send_limit < notification_count {
                msg_index = 0; //select negative msg from array
                severity = 2; //inform employees

                let message = get_message(
                    msg_index,
                    &settings.groups.memory.messages,
                    &max_item_crossed.label,
                );

                let l_msg = google_chat_config.build_msg(
                    severity,
                    &message,
                    priority,
                    &max_item_crossed.label,
                    &format!("{}", &max_item_crossed.target),
                );

                google_chat_config.send_chat_msg(l_msg);

                notified = true;
                notification_count = notification_count + 1;
            } else if max_target_crossed != 0 && send_limit > notification_count {
                //if notification count is more than send limit and still issue exist notify management

                msg_index = 0; //select negative msg from array
                severity = 1; //inform managers

                let message = get_message(
                    msg_index,
                    &settings.groups.memory.messages,
                    &max_item_crossed.label,
                );

                let l_msg = google_chat_config.build_msg(
                    severity,
                    &message,
                    priority,
                    &max_item_crossed.label,
                    &format!("{}", &max_item_crossed.target),
                );

                google_chat_config.send_chat_msg(l_msg);

                notified = true;
                notification_count = notification_count + 1;
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
