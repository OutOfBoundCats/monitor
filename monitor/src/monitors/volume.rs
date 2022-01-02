use std::{convert::TryInto, rc::Rc, sync::Arc, thread};

use systemstat::Platform;

use crate::{
    config::common::{Settings, VolumeItems},
    notifications::read_google_config::GoogleChatConfig,
};

use super::thread_sleep;

pub fn disk_capacity_usage() -> Vec<(u64, String)> {
    let sys = systemstat::System::new();

    let mut percent_free: Vec<(u64, String)> = Vec::new();

    match sys.mounts() {
        Ok(mounts) => {
            println!("\nMounts:");
            for mount in mounts.iter() {
                // println!(
                //     "{} ---{}---> {} (available {} of {})",
                //     mount.fs_mounted_from,
                //     mount.fs_type,
                //     mount.fs_mounted_on,
                //     mount.avail,
                //     mount.total
                // );

                let temnp_total_size = mount.total.as_u64();
                let temp_free_size = mount.avail.as_u64();
                let mut temp_percent_free;
                if temnp_total_size > 0 {
                    temp_percent_free = match (temp_free_size / temnp_total_size).try_into() {
                        Ok(value) => value,
                        Err(e) => 0,
                    };
                } else {
                    temp_percent_free = 0;
                }

                temp_percent_free = temp_percent_free * 100;
                let mounted_on = mount.fs_mounted_on.clone();

                percent_free.push((temp_percent_free, mounted_on));
            }
        }
        Err(x) => println!("\nMounts: error: {}", x),
    }

    percent_free
}

//starts disk monitoring
//#[tracing::instrument(skip(google_chat_config, settings))]
pub fn volume_monitor(
    google_chat_config: Arc<GoogleChatConfig>,
    settings: Settings,
    item: VolumeItems,
) {
    tracing::info!("Started Volume Monitor");

    let inactive_days = &settings.main.general.inactive_days;
    let inactive_times = &settings.main.general.inactive_times;
    let mut notified: bool = false;
    let mut msg_index: i32 = 0;
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
    match settings.groups.volumes.priority {
        Some(value) => priority = value,
        None => {
            priority = -1;
        }
    }

    let mut l_target = &item.target;
    let mut l_label = &item.label;

    loop {
        //sleep thread if current time falls between inactive time specified in json config
        thread_sleep(&inactive_times, &inactive_days);

        let disk_usage = disk_capacity_usage();

        for (disk_usage, mounted_on) in disk_usage {
            //if the mount is the one mentione in json then only compare

            if disk_usage > item.measurement.try_into().unwrap()
                && mounted_on == *l_target
                && notification_count <= send_limit
            {
                severity = 2;

                let message = get_message(
                    msg_index,
                    &settings.groups.volumes.messages,
                    item.measurement,
                    &l_label,
                );

                let l_msg =
                    google_chat_config.build_msg(severity, &message, priority, &l_label, &l_target);

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
            } else if disk_usage > item.measurement.try_into().unwrap()
                && mounted_on == *l_target
                && notification_count > send_limit
            {
                severity = 1;
                msg_index = 0;
                let message = get_message(
                    msg_index,
                    &settings.groups.volumes.messages,
                    item.measurement,
                    &l_label,
                );

                let l_msg =
                    google_chat_config.build_msg(severity, &message, priority, &l_label, &l_target);

                google_chat_config.send_chat_msg(l_msg);

                notification_count = 0;
                notified = false;
            } else if disk_usage < item.measurement.try_into().unwrap()
                && mounted_on == *l_target
                && notification_count > send_limit
            {
                notified = false;
                notification_count = 0;
                severity = 2;
                msg_index = 1; // select positive msg from array

                let message = get_message(
                    msg_index,
                    &settings.groups.volumes.messages,
                    item.measurement,
                    &l_label,
                );

                let l_msg =
                    google_chat_config.build_msg(severity, &message, priority, &l_label, &l_target);

                google_chat_config.send_chat_msg(l_msg);
                notification_count = 0;

                notified = false;

                thread::sleep(std::time::Duration::from_millis(
                    (item_sleep_mili).try_into().unwrap(),
                ));
            }
        }
    }
}

pub fn get_message(
    msg_index: i32,
    messages: &Vec<String>,
    measurement: i32,
    label: &String,
) -> String {
    let l_msg_index: usize = msg_index.clone().try_into().unwrap();
    let mut l_message = &messages[l_msg_index];
    let mut l_label: &String = label;

    let mut l_message_1;

    l_message_1 = l_message.replacen("{{}}", l_label, 1);
    l_message_1 = l_message.replacen("{{}}", format!("{}", &measurement).as_str(), 1);

    l_message_1

    // "raj".to_string()
}
