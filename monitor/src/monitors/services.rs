use std::sync::Arc;

use actix_web::http::header::IntoHeaderValue;
use run_script::ScriptOptions;

use crate::{config::common::Settings, notifications::read_google_config::GoogleChatConfig};

pub fn check_service() -> (i32, String) {
    let mut return_msg: String = "".to_string();
    let mut return_int: i32 = -1;

    let options = ScriptOptions::new();

    let args = vec![];
    let (code, output, error) = run_script::run(
        r#"systemctl is-active --quiet '+target+' 2>/dev/null
        "#,
        &args,
        &options,
    )
    .unwrap();

    let output_int: i32 = match output.parse() {
        Ok(value) => value,
        Err(err) => 2,
    };
    if output_int == 0 {
        return_msg = format!("Service is running fine");
        return_int = 0;
    } else if output_int > 0 {
        return_msg = format!("{} intsances of service detected", &output_int);
        return_int = 1;
    } else {
        return_msg = format!("Unexpected error occured while checking the service");
        return_int = 2;
    }

    (return_int, return_msg)
}

//service monitor
#[tracing::instrument(skip())]
pub fn service_monitor(google_chat_config: Arc<GoogleChatConfig>, settings: Settings) {
    let google_chat_mutex = google_chat;

    let mut notified: bool = false;
    let mut notification_count = 0;

    loop {
        let severity = 2;
        tracing::info!("Service monitor loop");
        thread_sleep(&inactive_times, &inactive_days);

        let item_sleep_mili = &item.item_sleep * 1000;

        let (service_status, service_msg) = services::check_service();

        if service_status == 0 {
            tracing::info!("Service functioning properly");
            tracing::info!("{}", service_msg);
        } else if service_status == 1 && notification_count <= item.send_limit {
            let message = google_chat_mutex.build_msg(
                &item,
                "ERROR",
                severity,
                format!("{}  ", &service_msg),
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

            tracing::error!("Warning Service not functioning properly");
            tracing::error!("{}", service_msg);
        } else if notification_count <= item.send_limit {
            let message =
                google_chat_mutex.build_msg(&item, "ERROR", severity, format!("{} ", &service_msg));

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

            tracing::error!("Error Service not functioning");
            tracing::error!("{}", service_msg);
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
