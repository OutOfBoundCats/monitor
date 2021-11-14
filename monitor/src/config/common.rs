use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use tracing::info;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub main: NotifyGeneral,
    pub groups: Vec<Services>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NotifyGeneral {
    pub general: General,
    pub notification: Notifications,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Notifications {
    pub version: String,
    pub send_limit: i32,
    pub first_wait: i32,
    pub wait_between: i32,
    pub priority: i32,
    pub item_sleep: i32,
    pub notification_ended_delay: i32,
    pub notify_wait: i32,
    pub url: String,
    pub button: String,
    pub notify: String,
    pub notify_not_relaxed: String,
    pub token: String,
    pub room: String,
    pub notify_model: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct General {
    // pub inactive_times: i32,
    // pub inactive_days: i32,
    pub log_messages_delay: i32,
    pub item_sleep: i32,
    pub service_sleep: i32,
    pub logfile: String,
    pub log: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Services {
    pub messages: String,

    pub priority: Option<i32>,

    pub first_wait: Option<i32>,

    pub wait_between: Option<i32>,

    pub send_limit: Option<i32>,

    pub item_sleep: Option<i32>,

    pub items: Vec<Items>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Items {
    pub name: String,
    pub label: String,
    pub target: String,
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
}

pub fn write_struct() {
    let notification = Notifications {
        version: "2.2".to_owned(),
        send_limit: 32,
        first_wait: 5,
        wait_between: 3,
        priority: 2,
        item_sleep: 2,
        notification_ended_delay: 5,
        notify_wait: 5,
        url: "loggernow.com".to_owned(),
        button: "button".to_owned(),
        notify: "notify".to_owned(),
        notify_not_relaxed: "notidfy_not_relaxxed".to_owned(),
        token: "xyz".to_owned(),
        room: "room".to_owned(),
        notify_model: "email".to_owned(),
    };
    let general = General {
        log_messages_delay: 2,
        item_sleep: 2,
        service_sleep: 2,
        logfile: "logfile".to_owned(),
        log: true,
    };

    let item = Items {
        name: "Server".to_owned(),
        label: "label".to_owned(),
        target: "target".to_owned(),
        priority: Some(2),
        first_wait: Some(2),
        wait_between: Some(2),
        send_limit: Some(2),
        item_sleep: Some(2),
    };
    let service = Services {
        messages: "message".to_owned(),
        priority: Some(2),
        first_wait: Some(2),
        wait_between: Some(2),
        send_limit: Some(2),
        item_sleep: Some(2),
        items: vec![item],
    };

    let main = NotifyGeneral {
        notification: notification,
        general: general,
    };

    let settings = Settings {
        main: main,
        groups: vec![service],
    };
    let serialized_setting = serde_json::to_string(&settings).unwrap();

    let path = "lines.json";

    // let mut output = File::create(path).unwrap();
    //fs::Write(output, serialized_setting.as_str()).unwrap();
    fs::write(path, &serialized_setting).expect("Unable to write file");

    //println!("{}", serialized_setting);
}

impl Settings {
    #[tracing::instrument(skip(self))]
    pub fn default_fill(&mut self) {
        //get values form notification section
        let general_send_limit = self.main.notification.send_limit;
        let general_first_wait = self.main.notification.first_wait;
        let general_wait_betwee = self.main.notification.wait_between;
        let general_priority = self.main.notification.priority;
        let general_item_sleep = self.main.notification.item_sleep;

        tracing::info!("item sleep from notification is {}", &general_item_sleep);

        //iterate over all groups in group
        for l in self.groups.iter_mut() {
            //println!("service priority is {}", &l.priority);
            //get group priority impute if its nulll from notification struct

            let group_priority = match l.priority {
                Some(value) => value,
                None => general_priority,
            };
            let group_first_wait = match l.first_wait {
                Some(value) => value,
                None => general_first_wait,
            };
            let group_wait_between = match l.wait_between {
                Some(value) => value,
                None => general_wait_betwee,
            };
            let group_send_limit = match l.send_limit {
                Some(value) => value,
                None => general_send_limit,
            };
            let group_item_sleep = match l.item_sleep {
                Some(value) => value,
                None => {
                    tracing::info!(
                        "item sleep from group is Null so imputing from general {}",
                        general_item_sleep
                    );
                    general_item_sleep
                }
            };

            //iterate over items in groups
            for item in l.items.iter_mut() {
                // 1. if item priority is None then take service priority
                let item_priority = match item.priority {
                    Some(value) => value,
                    None => group_priority,
                };
                //substitute the priority in struct
                item.priority = Some(item_priority);

                //2. if item first_wait is None then take service first_wait
                let item_first_wait = match item.first_wait {
                    Some(value) => value,
                    None => group_first_wait,
                };
                //substitute the priority in struct
                item.first_wait = Some(item_first_wait);

                //3. if item wait_between is None then take service service_wait_between
                let item_wait_between = match item.wait_between {
                    Some(value) => value,
                    None => group_wait_between,
                };
                //substitute the priority in struct
                item.wait_between = Some(item_wait_between);

                //4. if item send_limit is None then take service service_send_limit
                let item_send_limit = match item.send_limit {
                    Some(value) => value,
                    None => group_send_limit,
                };
                //substitute the priority in struct
                item.send_limit = Some(item_send_limit);

                //4. if item item_sleep is None then take service service_item_sleep
                let item_item_sleep = match item.item_sleep {
                    Some(value) => value,
                    None => {
                        info!(
                            "item sleep from item is Null so imputing from group {}",
                            &group_item_sleep
                        );
                        group_item_sleep
                    }
                };
                //substitute the priority in struct
                item.item_sleep = Some(item_item_sleep);
            }
        }
    }

    pub fn from_setting() -> Settings {
        write_struct();

        let data =
            fs::read_to_string("configurations/read_config.json").expect("Unable to read file");
        let mut serialised: Settings = serde_json::from_str(data.as_str()).unwrap();
        serialised.default_fill();
        let item_proprity = serialised.groups[0].items[0].item_sleep;
        //println!("new item priority {:?}", &item_proprity);
        serialised
    }
}
