use chrono::offset::LocalResult;
use chrono::prelude::*;

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use tracing::info;

//use crate::monitors::ping;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub main: NotifyGeneral,
    pub groups: Groups,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotifyGeneral {
    pub general: General,
    pub notification: Notifications,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Notifications {
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
    pub notify_model: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct General {
    pub version: String,
    pub inactive_times: Vec<(String, String)>,
    pub inactive_days: Vec<String>,
    pub log_messages_delay: i32,
    pub item_sleep: i32,
    pub service_sleep: i32,
    pub logfile: String,
    pub log: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Groups {
    pub services: services,
    pub volumes: volumes,
    pub pings: pings,
    pub memory: memory,
    pub cpu: cpu,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct services {
    pub messages: Vec<String>,
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub items: Vec<ServiceItems>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServiceItems {
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub label: String,
    pub target: String,
    pub command: String,
    pub output: String,
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct volumes {
    pub messages: Vec<String>,
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub items: Vec<VolumeItems>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VolumeItems {
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub label: String,
    pub target: String,
    pub measurement: i32, // in GB
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct pings {
    pub messages: Vec<String>,
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub items: Vec<PingItems>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PingItems {
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub label: String,
    pub target: String,
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct memory {
    pub messages: Vec<String>,
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub items: Vec<MemoryItems>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MemoryItems {
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub label: String,
    pub target: String,
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct cpu {
    pub messages: Vec<String>,
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub items: Vec<CpuItems>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CpuItems {
    pub priority: Option<i32>,
    pub first_wait: Option<i32>,
    pub wait_between: Option<i32>,
    pub send_limit: Option<i32>,
    pub item_sleep: Option<i32>,
    pub label: String,
    pub target: String,
    pub enabled: bool,
}

#[tracing::instrument]
pub fn write_struct() {
    let start_date = Local.ymd(2021, 11, 20).and_hms(9, 10, 11);
    let new_start_date = start_date.format("%d.%m.%Y %H:%M %P %:z");
    //tracing::info!("inactive date is {}", &new_start_date);

    let end_date = Local.ymd(2021, 8, 10).and_hms(10, 10, 11);
    let new_end_date = end_date.format("%d.%m.%Y %H:%M %P %:z");
    //tracing::info!("inactive date is {}", &new_end_date);

    let inactive_day1 = Local.ymd(2021, 11, 20).and_hms(0, 0, 0);
    let new_inactive_day1 = inactive_day1.format("%d.%m.%Y");
    //tracing::info!("inactive date is {}", &new_inactive_day1);

    let notification = Notifications {
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
        notify_model: "email".to_owned(),
    };
    let general = General {
        version: "1.0.1".to_string(),
        inactive_times: vec![(start_date.to_string(), end_date.to_string())],
        inactive_days: vec![inactive_day1.to_string()],
        log_messages_delay: 2,
        item_sleep: 2,
        service_sleep: 2,
        logfile: "logfile".to_owned(),
        log: true,
    };

    //write services
    let service_item1 = ServiceItems {
        priority: Some(3),
        first_wait: Some(30),
        wait_between: Some(30),
        send_limit: Some(3),
        item_sleep: Some(30),
        label: "cron".to_string(),
        target: "cron".to_string(),
        command: "".to_string(),
        output: "2".to_string(),
        enabled: true,
    };

    let l_services = services {
        messages: vec![
            "Service {{label}} not running".to_string(),
            "Service {{label}} is active".to_string(),
        ],
        priority: Some(3),
        first_wait: Some(10),
        wait_between: Some(15),
        send_limit: Some(3),
        item_sleep: Some(30),
        items: vec![service_item1],
    };

    //make volume
    let volume_item = VolumeItems {
        priority: Some(3),
        first_wait: Some(30),
        wait_between: Some(30),
        send_limit: Some(3),
        item_sleep: Some(30),
        label: "root".to_string(),
        target: "/".to_string(),
        measurement: 2, // in GB
        enabled: true,
    };

    let l_volume = volumes {
        messages: vec![
            "Volume capacity low in {{label}}. Under {{measurement}}".to_string(),
            "Volume size is stable".to_string(),
        ],
        priority: Some(3),
        first_wait: Some(10),
        wait_between: Some(15),
        send_limit: Some(3),
        item_sleep: Some(30),
        items: vec![volume_item],
    };

    //make ping
    let ping_item = PingItems {
        priority: Some(3),
        first_wait: Some(30),
        wait_between: Some(30),
        send_limit: Some(3),
        item_sleep: Some(30),
        label: "google".to_string(),
        target: "www.google.com:443".to_string(),
        enabled: true,
    };

    let l_pings = pings {
        messages: vec![
            "Host {{label}} not responding to ping".to_string(),
            "Pîng sucess".to_string(),
        ],
        priority: Some(3),
        first_wait: Some(10),
        wait_between: Some(15),
        send_limit: Some(3),
        item_sleep: Some(30),
        items: vec![ping_item],
    };

    // make memory

    let memory_item = MemoryItems {
        priority: Some(3),
        first_wait: Some(30),
        wait_between: Some(30),
        send_limit: Some(3),
        item_sleep: Some(30),
        label: "critical".to_string(),
        target: "20".to_string(),
        enabled: true,
    };

    let l_memory = memory {
        messages: vec![
            "Memory usage notice ({{label}})".to_string(),
            "Memory usage is under the limit".to_string(),
        ],
        priority: Some(3),
        first_wait: Some(10),
        wait_between: Some(15),
        send_limit: Some(3),
        item_sleep: Some(30),
        items: vec![memory_item],
    };

    //make cpu

    let cpu_item = CpuItems {
        priority: Some(3),
        first_wait: Some(30),
        wait_between: Some(30),
        send_limit: Some(3),
        item_sleep: Some(30),
        label: "critical".to_string(),
        target: "100".to_string(),
        enabled: true,
    };

    let l_cpu = cpu {
        messages: vec![
            "CPU usage notice ({{label}})".to_string(),
            "CPU usage is valid".to_string(),
        ],
        priority: Some(3),
        first_wait: Some(10),
        wait_between: Some(15),
        send_limit: Some(3),
        item_sleep: Some(30),
        items: vec![cpu_item],
    };

    let groups = Groups {
        services: l_services,
        volumes: l_volume,
        pings: l_pings,
        memory: l_memory,
        cpu: l_cpu,
    };

    let main = NotifyGeneral {
        notification: notification,
        general: general,
    };

    let settings = Settings {
        main: main,
        groups: groups,
    };
    let serialized_setting = serde_json::to_string(&settings).unwrap();

    let path = "lines.json";

    fs::write(path, &serialized_setting).expect("Unable to write file");
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

        //iterate over all groups in group

        // impute values in services
        let services_group_first_wait = match self.groups.services.first_wait {
            Some(v) => v,
            None => general_first_wait,
        };
        let services_group_send_limit = match self.groups.services.send_limit {
            Some(v) => v,
            None => general_send_limit,
        };
        let services_group_wait_between = match self.groups.services.wait_between {
            Some(v) => v,
            None => general_wait_betwee,
        };
        let services_group_priority = match self.groups.services.priority {
            Some(v) => v,
            None => general_priority,
        };
        let services_item_sleep = match self.groups.services.item_sleep {
            Some(v) => v,
            None => general_item_sleep,
        };

        for item in self.groups.services.items {
            item.first_wait = Some(match item.first_wait {
                Some(v) => v,
                None => services_group_first_wait,
            });

            item.send_limit = Some(match item.send_limit {
                Some(v) => v,
                None => services_group_send_limit,
            });

            item.wait_between = Some(match item.wait_between {
                Some(v) => v,
                None => services_group_wait_between,
            });

            item.priority = Some(match item.priority {
                Some(v) => v,
                None => services_group_priority,
            });

            item.item_sleep = Some(match item.item_sleep {
                Some(v) => v,
                None => services_item_sleep,
            });
        }

        // impute values in volumes

        let volumes_group_first_wait = match self.groups.volumes.first_wait {
            Some(v) => v,
            None => general_first_wait,
        };
        let volumes_group_send_limit = match self.groups.volumes.send_limit {
            Some(v) => v,
            None => general_send_limit,
        };
        let volumes_group_wait_between = match self.groups.volumes.wait_between {
            Some(v) => v,
            None => general_wait_betwee,
        };
        let volumes_group_priority = match self.groups.volumes.priority {
            Some(v) => v,
            None => general_priority,
        };
        let volumes_item_sleep = match self.groups.volumes.item_sleep {
            Some(v) => v,
            None => general_item_sleep,
        };

        for item in self.groups.volumes.items {
            item.first_wait = Some(match item.first_wait {
                Some(v) => v,
                None => volumes_group_first_wait,
            });

            item.send_limit = Some(match item.send_limit {
                Some(v) => v,
                None => volumes_group_send_limit,
            });

            item.wait_between = Some(match item.wait_between {
                Some(v) => v,
                None => volumes_group_wait_between,
            });

            item.priority = Some(match item.priority {
                Some(v) => v,
                None => volumes_group_priority,
            });

            item.item_sleep = Some(match item.item_sleep {
                Some(v) => v,
                None => volumes_item_sleep,
            });
        }

        // impute pings

        let pings_group_first_wait = match self.groups.pings.first_wait {
            Some(v) => v,
            None => general_first_wait,
        };
        let pings_group_send_limit = match self.groups.pings.send_limit {
            Some(v) => v,
            None => general_send_limit,
        };
        let pings_group_wait_between = match self.groups.pings.wait_between {
            Some(v) => v,
            None => general_wait_betwee,
        };
        let pings_group_priority = match self.groups.pings.priority {
            Some(v) => v,
            None => general_priority,
        };
        let pings_item_sleep = match self.groups.pings.item_sleep {
            Some(v) => v,
            None => general_item_sleep,
        };

        for item in self.groups.pings.items {
            item.first_wait = Some(match item.first_wait {
                Some(v) => v,
                None => pings_group_first_wait,
            });

            item.send_limit = Some(match item.send_limit {
                Some(v) => v,
                None => pings_group_send_limit,
            });

            item.wait_between = Some(match item.wait_between {
                Some(v) => v,
                None => pings_group_wait_between,
            });

            item.priority = Some(match item.priority {
                Some(v) => v,
                None => pings_group_priority,
            });

            item.item_sleep = Some(match item.item_sleep {
                Some(v) => v,
                None => pings_item_sleep,
            });
        }

        // impute memory

        let memory_group_first_wait = match self.groups.memory.first_wait {
            Some(v) => v,
            None => general_first_wait,
        };
        let memory_group_send_limit = match self.groups.memory.send_limit {
            Some(v) => v,
            None => general_send_limit,
        };
        let memory_group_wait_between = match self.groups.memory.wait_between {
            Some(v) => v,
            None => general_wait_betwee,
        };
        let memory_group_priority = match self.groups.memory.priority {
            Some(v) => v,
            None => general_priority,
        };
        let memory_item_sleep = match self.groups.memory.item_sleep {
            Some(v) => v,
            None => general_item_sleep,
        };

        for item in self.groups.memory.items {
            item.first_wait = Some(match item.first_wait {
                Some(v) => v,
                None => memory_group_first_wait,
            });

            item.send_limit = Some(match item.send_limit {
                Some(v) => v,
                None => memory_group_send_limit,
            });

            item.wait_between = Some(match item.wait_between {
                Some(v) => v,
                None => memory_group_wait_between,
            });

            item.priority = Some(match item.priority {
                Some(v) => v,
                None => memory_group_priority,
            });

            item.item_sleep = Some(match item.item_sleep {
                Some(v) => v,
                None => memory_item_sleep,
            });
        }

        // impute cpu

        let cpu_group_first_wait = match self.groups.cpu.first_wait {
            Some(v) => v,
            None => general_first_wait,
        };
        let cpu_group_send_limit = match self.groups.cpu.send_limit {
            Some(v) => v,
            None => general_send_limit,
        };
        let cpu_group_wait_between = match self.groups.cpu.wait_between {
            Some(v) => v,
            None => general_wait_betwee,
        };
        let cpu_group_priority = match self.groups.cpu.priority {
            Some(v) => v,
            None => general_priority,
        };
        let cpu_item_sleep = match self.groups.cpu.item_sleep {
            Some(v) => v,
            None => general_item_sleep,
        };

        for item in self.groups.cpu.items {
            item.first_wait = Some(match item.first_wait {
                Some(v) => v,
                None => cpu_group_first_wait,
            });

            item.send_limit = Some(match item.send_limit {
                Some(v) => v,
                None => cpu_group_send_limit,
            });

            item.wait_between = Some(match item.wait_between {
                Some(v) => v,
                None => cpu_group_wait_between,
            });

            item.priority = Some(match item.priority {
                Some(v) => v,
                None => cpu_group_priority,
            });

            item.item_sleep = Some(match item.item_sleep {
                Some(v) => v,
                None => cpu_item_sleep,
            });
        }
    }

    #[tracing::instrument]
    pub fn from_setting() -> Settings {
        write_struct();
        tracing::info!("wrote sample configuration file");

        let mut data: String;
        let res = fs::read_to_string("configurations/read_config.json");

        match res {
            Ok(value) => data = value.to_string(),
            Err(err) => {
                tracing::error!(
                    "Error occured while reading configuratioon file => {}",
                    &err
                );
                panic!(
                    "Error occured while reading configuratioon file => {}",
                    &err
                );
            }
        }
        tracing::info!("Configuration file read succesfully");

        let mut serialised: Settings = serde_json::from_str(data.as_str()).unwrap();
        serialised.default_fill();
        tracing::info!("Filled item data from parent data");

        serialised
    }
}
