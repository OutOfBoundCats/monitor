use std::{convert::TryInto, sync::Arc, thread};

//pub use loggernow_cpu::cpu::get_percentage_cpu_usage;
use sysinfo::{ProcessorExt, System, SystemExt};
use systemstat::{saturating_sub_bytes, Platform};

use crate::{config::common::Settings, notifications::read_google_config::GoogleChatConfig};

use super::thread_sleep;

// pub fn cpu_usage() -> f32 {
//     let mut s = System::new_all();
//     let mut sum = 0.0;
//     let mut count = 0.0;
//     for processor in s.processors() {
//         sum = sum + processor.cpu_usage();
//         count = count + 1.0;
//     }
//     let average_cpu = sum / count;
//     average_cpu
// }

pub fn cpu_usage() -> f32 {
    let sys = systemstat::System::new();
    let cpu_load = sys.cpu_load_aggregate().unwrap();
    let mut cpu_usage = 0.0;
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            println!("\nMeasuring CPU load...");
            std::thread::sleep(systemstat::Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            cpu_usage = 100.0 - cpu.idle * 100.0;
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }
    cpu_usage
}

//starts CPU monitoring
#[tracing::instrument(skip(google_chat_config, settings))]
pub fn cpu_monitor(google_chat_config: Arc<GoogleChatConfig>, settings: Settings) {
    tracing::info!("Started CPU Monitor");

    let inactive_days = settings.main.general.inactive_days;
    let inactive_times = settings.main.general.inactive_times;
    let notified: bool = false;
    let mut msg_index: i32;
    let mut notification_count = 0;
    let mut send_limit: i32;

    match settings.groups.cpu.send_limit {
        Some(value) => {
            send_limit = value;
        }
        None => {
            send_limit = settings.main.notification.send_limit;
        }
    }

    loop {
        let severity = 2;

        //sleep thread if current time falls between inactive time specified in json config
        thread_sleep(&inactive_times, &inactive_days);
        let item_sleep_mili: i32;
        match settings.groups.cpu.item_sleep {
            Some(value) => {
                item_sleep_mili = value * 1000;
            }
            None => {
                tracing::error!("Error in getting the cpu group item_sleep time");
                item_sleep_mili = settings.main.notification.item_sleep * 1000;
            }
        }

        let cpu_usage = cpu_usage();

        //find which item with max threashold is getting crossed
        let vec_index: Vec<i32> = vec![];
        let vec_value: Vec<i32> = vec![];
        let mut i = 0;
        for item in settings.groups.cpu.items {
            if cpu_usage > item.target.parse().unwrap() {
                vec_index.push(i);
                let target = item.target.clone().parse().unwrap();
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

        if max_value != -1 {
            let max_index = vec_value.iter().position(|&r| r == max_value).unwrap();
            let item_index: usize = vec_index[max_index].try_into().unwrap();

            //got the item with highest threshold being croseed by cpu usage
            let l_item = settings.groups.cpu.items[item_index];

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

            //for 1st msg wait for first wait
            if notified == false {
                msg_index = 0; //select negative msg from array
                severity = 2; //inform employees

                let l_msg = google_chat_config.build_msg(
                    severity,
                    settings.groups.cpu.messages,
                    msg_index,
                    settings.groups.cpu.priority,
                    Some(l_item.label),
                    Some(l_item.target),
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

                let l_msg = google_chat_config.build_msg(
                    severity,
                    settings.groups.cpu.messages,
                    msg_index,
                    settings.groups.cpu.priority,
                    Some(l_item.label),
                    Some(l_item.target),
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

                let l_msg = google_chat_config.build_msg(
                    severity,
                    settings.groups.cpu.messages,
                    msg_index,
                    settings.groups.cpu.priority,
                    Some(l_item.label),
                    Some(l_item.target),
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

            let l_msg = google_chat_config.build_msg(
                severity,
                settings.groups.cpu.messages,
                msg_index,
                settings.groups.cpu.priority,
                None,
                None,
            );

            google_chat_config.send_chat_msg(l_msg);
            notification_count = 0;

            notified = false;

            thread::sleep(std::time::Duration::from_millis(
                (item_sleep_mili).try_into().unwrap(),
            ));
        }
    }
}
