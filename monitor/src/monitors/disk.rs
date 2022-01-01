use std::{convert::TryInto, sync::Arc};

use systemstat::{saturating_sub_bytes, ByteSize, Platform};

use crate::{config::common::Settings, notifications::read_google_config::GoogleChatConfig};

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
#[tracing::instrument(skip(google_chat_config, settings))]
pub fn volume_monitor(google_chat_config: Arc<GoogleChatConfig>, settings: Settings) {
    let inactive_days = settings.main.general.inactive_days;
    let inactive_times = settings.main.general.inactive_times;
    let notified: bool = false;
    let mut msg_index: i32;
    let mut notification_count = 0;
    let mut send_limit: i32;

    loop {
        let severity = 2;

        //sleep thread if current time falls between inactive time specified in json config
        thread_sleep(&inactive_times, &inactive_days);

        for item in settings.groups.volumes.items {
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
        }
    }

    // let google_chat_mutex = google_chat;

    // let mut notified: bool = false;
    // let mut notification_count = 0;

    // loop {
    //     let severity = 2;
    //     tracing::info!("Disk monitor loop");
    //     thread_sleep(&inactive_times, &inactive_days);

    //     let item_sleep_mili = &item.item_sleep * 1000;

    //     let disk_usage = disk::disk_capacity_usage();

    //     let mut messsage: String = "".to_string();

    //     for (disk_usage, mounted_on) in disk_usage {
    //         if disk_usage > 90 {
    //             tracing::error!(
    //                 "Mounted disk  on path {} is {} full",
    //                 &mounted_on,
    //                 &disk_usage
    //             );

    //             let temp = format!(
    //                 "Mounted disk  on path {} is {} %  full <br>",
    //                 &mounted_on, &disk_usage
    //             );

    //             messsage.push_str(&temp);
    //         }
    //     }

    //     let msg_len: usize = 0;
    //     if &messsage.len() != &msg_len && notification_count <= item.send_limit {
    //         let message =
    //             google_chat_mutex.build_msg(&item, "ERROR", severity, format!("{}", &messsage));

    //         let res = google_chat_mutex.send_chat_msg(message);

    //         notified = true;
    //         notification_count = notification_count + 1;
    //         if notification_count == 1 {
    //             thread::sleep(std::time::Duration::from_millis(
    //                 (item.first_wait * 1000).try_into().unwrap(),
    //             ));
    //         } else {
    //             thread::sleep(std::time::Duration::from_millis(
    //                 (item.wait_between * 1000).try_into().unwrap(),
    //             ));
    //         }
    //     } else {
    //         notification_count = 0;
    //         notified = false;
    //     }

    //     if notified == false {
    //         thread::sleep(std::time::Duration::from_millis(
    //             item_sleep_mili.try_into().unwrap(),
    //         ));
    //     }
    // }
}
