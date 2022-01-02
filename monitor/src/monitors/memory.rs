use std::{convert::TryInto, sync::Arc, thread};

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

//starts memory monitoring
#[tracing::instrument(skip(google_chat_config, settings))]
pub fn memory_monitor(google_chat_config: Arc<GoogleChatConfig>, settings: Settings) {
    tracing::info!("Started Memory Monitor");

    let inactive_days = settings.main.general.inactive_days.clone();
    let inactive_times = settings.main.general.inactive_times.clone();
    let mut notified: bool = false;
    let mut msg_index: i32;
    let mut notification_count = 0;
    let mut send_limit: i32;
    let mut severity = 2;

    match settings.groups.memory.send_limit {
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

        match settings.groups.memory.item_sleep {
            Some(value) => {
                item_sleep_mili = value * 1000;
            }
            None => {
                item_sleep_mili = settings.main.notification.item_sleep;
                tracing::error!("Error in getting the cpu group item_sleep time");
            }
        }

        let (used_memory, total_memory, err) = memory_usage();

        let memory_used_percentage = used_memory / total_memory * 100;

        //find which item with max threashold is getting crossed
        let mut vec_index: Vec<i32> = vec![];
        let mut vec_value: Vec<i32> = vec![];
        let mut i: usize = 0;

        let l_setting = settings.clone();
        let l_setting1 = settings.clone();
        for item in l_setting.groups.memory.items {
            if memory_used_percentage > item.target.parse().unwrap() {
                vec_index.push(i.try_into().unwrap());
                let target = l_setting1.groups.memory.items[i]
                    .target
                    .clone()
                    .parse()
                    .unwrap();
                vec_value.push(target);
            }

            i = i + 1;
        }
        //check the maximum threadshold which is gettign breached by current cpu usage
        let mut max_value: i32 = -1;
        match vec_value.iter().max() {
            Some(value) => {
                max_value = *value;
            }
            None => {
                max_value = -1;
            }
        }

        //find minimum threashold in items
        let mut vec_min_index: Vec<i32> = vec![];
        let mut vec_min_value: Vec<i32> = vec![];
        let mut i = 0;
        for item in &settings.groups.memory.items {
            vec_min_index.push(i);
            let target = item.target.clone().parse().unwrap();
            vec_min_value.push(target);

            i = i + 1;
        }
        //check the maximum threadshold which is gettign breached by current cpu usage
        let mut min_value: i32 = -1;
        match vec_min_value.iter().min() {
            Some(value) => {
                min_value = *value;
            }
            None => {
                min_value = -1;
            }
        }
        let mut min_index;
        let mut item_min_index: usize = usize::MAX;
        if min_value != -1 {
            min_index = vec_value.iter().position(|&r| r == min_value).unwrap();
            item_min_index = vec_index[min_index].try_into().unwrap();
        }

        if max_value != -1 {
            let max_index = vec_value.iter().position(|&r| r == max_value).unwrap();
            let item_index: usize = vec_index[max_index].try_into().unwrap();

            //got the item with highest threshold being croseed by cpu usage
            let l_item = &settings.groups.memory.items[item_index];

            let mut l_first_wait;
            match l_item.first_wait {
                Some(value) => {
                    l_first_wait = value;
                }
                None => {
                    l_first_wait = settings.main.notification.first_wait;
                }
            };

            let mut l_wait_between;
            match l_item.wait_between {
                Some(value) => {
                    l_wait_between = value;
                }
                None => {
                    l_wait_between = settings.main.notification.wait_between;
                }
            };

            let mut priority;
            match settings.groups.memory.priority {
                Some(value) => {
                    priority = value;
                }
                None => {
                    priority = -1;
                }
            }

            //for 1st msg wait for first wait
            if notified == false {
                msg_index = 0; //select negative msg from array
                severity = 2; //inform employees

                let message =
                    get_message(msg_index, &settings.groups.memory.messages, &l_item.label);

                let l_msg = google_chat_config.build_msg(
                    severity,
                    &message,
                    priority,
                    &l_item.label,
                    &l_item.target,
                );

                google_chat_config.send_chat_msg(l_msg);

                notified = true;

                notification_count = notification_count + 1; // increament notification count

                thread::sleep(std::time::Duration::from_millis(
                    (l_first_wait * 1000).try_into().unwrap(),
                ));
            } else if notified == true && notification_count <= send_limit {
                msg_index = 0; //select negative msg from array
                severity = 2; //inform employees

                let message =
                    get_message(msg_index, &settings.groups.memory.messages, &l_item.label);

                let l_msg = google_chat_config.build_msg(
                    severity,
                    &message,
                    priority,
                    &l_item.label,
                    &l_item.target,
                );

                google_chat_config.send_chat_msg(l_msg);

                notified = true;

                notification_count = notification_count + 1; // increament notification count

                thread::sleep(std::time::Duration::from_millis(
                    (l_wait_between * 1000).try_into().unwrap(),
                ));
            } else if notified == true && notification_count > send_limit {
                //notify management

                msg_index = 0; // select negative msg from array
                severity = 1; //inform management

                let message =
                    get_message(msg_index, &settings.groups.memory.messages, &l_item.label);

                let l_msg = google_chat_config.build_msg(
                    severity,
                    &message,
                    priority,
                    &l_item.label,
                    &l_item.target,
                );

                google_chat_config.send_chat_msg(l_msg);

                //for subsequent messages wait for wait between
                thread::sleep(std::time::Duration::from_millis(
                    (l_wait_between * 1000).try_into().unwrap(),
                ));

                notification_count = 0;

                notified = true;
            }
        }

        //if suers were already notified before and cpu usage is normal now notify with +ve message

        if max_value == -1 && notified == true {
            notified = false;
            notification_count = 0;
            severity = 2;
            msg_index = 1; // select positive msg from array

            let priority;
            match settings.groups.memory.priority {
                Some(value) => {
                    priority = value;
                }
                None => {
                    priority = -1;
                }
            }

            let l_min_item = &settings.groups.memory.items[item_min_index];

            let message = get_message(
                msg_index,
                &settings.groups.memory.messages,
                &l_min_item.label,
            );

            let l_msg = google_chat_config.build_msg(
                severity,
                &message,
                priority,
                &l_min_item.label,
                &l_min_item.target,
            );

            google_chat_config.send_chat_msg(l_msg);

            notification_count = 0;

            notified = false;

            //for subsequent messages wait for wait between
            thread::sleep(std::time::Duration::from_millis(
                (item_sleep_mili).try_into().unwrap(),
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
