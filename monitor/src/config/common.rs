use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub main: NotifyGeneral,
    pub groups: Groups,
}

#[derive(Deserialize, Serialize)]
pub struct NotifyGeneral {
    pub notification: Notifications,
    pub general: General,
}
#[derive(Deserialize, Serialize)]
pub struct Notifications {
    pub version: String,
    pub send_limit: i32,
    pub first_wait: i32,
    pub wait_between: i32,
    pub priority: i32,
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

#[derive(Deserialize, Serialize)]
pub struct General {
    pub log_messages_delay: i32,
    pub item_sleep: i32,
    pub service_sleep: i32,
    pub logfile: String,
    pub log: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Groups {
    pub list: Vec<Services>,
}

#[derive(Deserialize, Serialize)]
pub struct Services {
    pub messages: String,
    #[serde(default = "default_priority")]
    pub priority: i32,

    #[serde(default = "default_first_wait")]
    pub first_wait: i32,

    #[serde(default = "default_wait_between")]
    pub wait_between: i32,

    #[serde(default = "default_send_limit")]
    pub send_limit: i32,

    #[serde(default = "default_item_sleep")]
    pub item_sleep: i32,
    pub items: Vec<Items>,
}
fn default_priority() -> i32 {
    2
}
fn default_first_wait() -> i32 {
    2
}
fn default_wait_between() -> i32 {
    2
}
fn default_send_limit() -> i32 {
    2
}
fn default_item_sleep() -> i32 {
    2
}

#[derive(Deserialize, Reflect, Serialize)]
pub struct Items {
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
        priority: 2,
        first_wait: 2,
        wait_between: 2,
        send_limit: 2,
        item_sleep: 2,
        items: vec![item],
    };
    let group = Groups {
        list: vec![service],
    };
    let main = NotifyGeneral {
        notification: notification,
        general: general,
    };

    let settings = Settings {
        main: main,
        groups: group,
    };
    let serialized_setting = serde_json::to_string(&settings).unwrap();

    let path = "lines.json";

    // let mut output = File::create(path).unwrap();
    //fs::Write(output, serialized_setting.as_str()).unwrap();
    fs::write(path, &serialized_setting).expect("Unable to write file");

    //println!("{}", serialized_setting);
}

impl Settings {
    pub fn default_fill(&mut self) {
        //iterate over all services in group
        for l in self.groups.list.iter_mut() {
            //println!("service priority is {}", &l.priority);
            //get service priority
            let service_priority = l.priority;
            let service_first_wait = l.first_wait;
            let service_wait_between = l.wait_between;
            let service_send_limit = l.send_limit;
            let service_item_sleep = l.item_sleep;

            //iterate over items in service
            for item in l.items.iter_mut() {
                // 1. if item priority is None then take service priority
                let item_priority = match item.priority {
                    Some(value) => value,
                    None => service_priority,
                };
                //substitute the priority in struct
                item.priority = Some(item_priority);

                //2. if item first_wait is None then take service first_wait
                let item_first_wait = match item.first_wait {
                    Some(value) => value,
                    None => service_first_wait,
                };
                //substitute the priority in struct
                item.first_wait = Some(item_first_wait);

                //3. if item wait_between is None then take service service_wait_between
                let item_wait_between = match item.wait_between {
                    Some(value) => value,
                    None => service_wait_between,
                };
                //substitute the priority in struct
                item.wait_between = Some(item_wait_between);

                //4. if item send_limit is None then take service service_send_limit
                let item_send_limit = match item.send_limit {
                    Some(value) => value,
                    None => service_send_limit,
                };
                //substitute the priority in struct
                item.send_limit = Some(item_send_limit);

                //4. if item item_sleep is None then take service service_item_sleep
                let item_item_sleep = match item.item_sleep {
                    Some(value) => value,
                    None => service_item_sleep,
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
        let item_proprity = serialised.groups.list[0].items[0].priority;
        //println!("new item priority {:?}", &item_proprity);
        serialised
    }
}
