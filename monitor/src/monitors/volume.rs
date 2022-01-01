use std::{convert::TryInto, sync::Arc, thread};

use systemstat::{saturating_sub_bytes, ByteSize, Platform};

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
#[tracing::instrument(skip(google_chat_config, settings))]
pub fn volume_monitor(
    google_chat_config: Arc<GoogleChatConfig>,
    settings: Settings,
    item: VolumeItems,
) {
    tracing::info!("Started Volume Monitor");

    let inactive_days = settings.main.general.inactive_days;
    let inactive_times = settings.main.general.inactive_times;
    let notified: bool = false;
    let mut msg_index: i32;
    let mut notification_count = 0;
    let mut send_limit: i32;

    match item.send_limit {
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

        let disk_usage = disk_capacity_usage();

        for (disk_usage, mounted_on) in disk_usage {
            //if the mount is the one mentione in json then only compare
            if disk_usage > item.measurement.try_into().unwrap()
                && mounted_on == item.target
                && notification_count <= send_limit
            {
                severity = 2;
                tracing::error!("");

                let l_msg = google_chat_config.build_msg(
                    severity,
                    settings.groups.volumes.messages,
                    msg_index,
                    settings.groups.volumes.priority,
                    Some(item.label),
                    Some(item.target),
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
            } else if disk_usage > item.measurement.try_into().unwrap()
                && mounted_on == item.target
                && notification_count > send_limit
            {
                severity = 1;
                let l_msg = google_chat_config.build_msg(
                    severity,
                    settings.groups.volumes.messages,
                    msg_index,
                    settings.groups.volumes.priority,
                    Some(item.label),
                    Some(item.target),
                );

                google_chat_config.send_chat_msg(l_msg);

                notification_count = 0;
                notified = false;
            }
        }
    }
}
