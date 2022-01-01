use std::{convert::TryInto, sync::Arc, thread};

use systemstat::{saturating_sub_bytes, ByteSize, Platform};

use crate::{monitors::thread_sleep, notifications::read_google_config::GoogleChatConfig};

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
#[tracing::instrument(skip())]
pub fn memory_monitor(google_chat_config: Arc<GoogleChatConfig>, settings: Settings) {
    let google_chat_mutex = google_chat;

    let mut notified: bool = false;
    let mut notification_count = 0;

    loop {
        let severity = 2;
        tracing::info!("Memory monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let (used_memory, total_memory, err) = memory_usage();
        let mut memory_usage: i64 = 0;
        if err == false {
            memory_usage = ((used_memory / total_memory) * 100).try_into().unwrap();
        } else {
            memory_usage = -2;
        }

        tracing::info!("memory usage is {}", &memory_usage);
        if memory_usage > 10 && notification_count <= item.send_limit {
            let message = google_chat_mutex.build_msg(
                &item,
                "ERROR",
                severity,
                format!("Memory usage very high at {} ", &memory_usage),
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

            tracing::error!("Memory usage very high at {} ", &memory_usage);
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
