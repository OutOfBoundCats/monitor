use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

extern crate serde;
extern crate serde_value;

use serde::de::DeserializeOwned;
use serde_value::Value;

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

#[derive(Deserialize, Serialize, Reflect)]
pub struct Services {
    pub messages: String,
    pub priority: i32,
    pub first_wait: i32,
    pub wait_between: i32,
    pub send_limit: i32,
    pub item_sleep: i32,
    pub items: Vec<Items>,
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

    let item1 = Items {
        label: "item1".to_owned(),
        target: "target".to_owned(),
        priority: Some(2),
        first_wait: Some(2),
        wait_between: Some(2),
        send_limit: Some(2),
        item_sleep: Some(2),
    };
    let item2 = Items {
        label: "item2".to_owned(),
        target: "target".to_owned(),
        priority: Some(2),
        first_wait: Some(2),
        wait_between: Some(2),
        send_limit: Some(2),
        item_sleep: Some(2),
    };
    let item3 = Items {
        label: "item1".to_owned(),
        target: "target".to_owned(),
        priority: Some(2),
        first_wait: Some(2),
        wait_between: Some(2),
        send_limit: Some(2),
        item_sleep: Some(2),
    };
    let item4 = Items {
        label: "item2".to_owned(),
        target: "target".to_owned(),
        priority: Some(2),
        first_wait: Some(2),
        wait_between: Some(2),
        send_limit: Some(2),
        item_sleep: Some(2),
    };
    let service = Services {
        messages: "service1".to_owned(),
        priority: 2,
        first_wait: 2,
        wait_between: 2,
        send_limit: 2,
        item_sleep: 2,
        items: vec![item1, item2],
    };
    let service2 = Services {
        messages: "service2".to_owned(),
        priority: 2,
        first_wait: 2,
        wait_between: 2,
        send_limit: 2,
        item_sleep: 2,
        items: vec![item3, item4],
    };
    let group = Groups {
        list: vec![service, service2],
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

    fs::write(path, &serialized_setting).expect("Unable to write file");

    //println!("{}", serialized_setting);
}

impl Settings {
    pub fn put_default_from_head(&mut self) {
        for item in self.groups.list[0].items.iter_mut() {
            let a = &item.label;
            println!("from mut iter for {:?}", a);
        }

        // for (i, value: &Reflect) in self.groups.list[0].items[0].iter_fields().enumerate() {
        //     let field_name = self.groups.list[0].items[0].name_at(i).unwrap();
        //     println!("columns is {} and value is ", &field_name);
        //     match value.downcast_ref::<u32>() {
        //         Some(value) => println!("{} is a u32 with the value: {}", field_name, value),
        //         None => println!("none"),
        //     };
        // }
    }
}

fn get_field_by_name<T, R>(data: T, field: &str) -> R
where
    T: Serialize,
    R: DeserializeOwned,
{
    let mut map = match serde_value::to_value(data) {
        Ok(Value::Map(map)) => map,
        _ => panic!("expected a struct"),
    };

    let key = Value::String(field.to_owned());
    let value = match map.remove(&key) {
        Some(value) => value,
        None => panic!("no such field"),
    };

    match R::deserialize(value) {
        Ok(r) => r,
        Err(_) => panic!("wrong type?"),
    }
}
