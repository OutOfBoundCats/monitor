use chrono::offset::LocalResult;
use chrono::prelude::*;

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use tracing::info;

#[derive(Deserialize, Serialize, Debug)]
pub struct GoogleChatConfig {
    pub general: General,
    pub groups: Vec<Item>,
    pub chaturl: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct General {
    pub management: Vec<String>,
    pub employees: Vec<String>,
    pub good_msg: String,
    pub error_sev2: String,
    pub error_sev1: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub name: String,
    pub management: Option<Vec<String>>,
    pub employees: Option<Vec<String>>,
}

#[tracing::instrument]
pub fn write_struct() {
    let item1 = Item {
        name: "CPU".to_string(),
        management: Some(vec!["107583083364112988124".to_string()]),
        employees: Some(vec!["107583083364112988124".to_string()]),
    };

    let general = General{
         management: vec!["107583083364112988124".to_string()],
         employees: vec!["107583083364112988124".to_string()],
         good_msg: "https://ak.picdn.net/shutterstock/videos/1068883754/thumb/11.jpg".to_string(),
         error_sev2: "https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/160/google/110/heavy-exclamation-mark-symbol_2757.png".to_string(),
         error_sev1: "https://hotemoji.com/images/dl/u/double-exclamation-mark-emoji-by-google.png".to_string(),
    };

    let googleChatConfig = GoogleChatConfig {
        general: general,
        groups: vec![item1],
        chaturl: Some("https://chat.googleapis.com/v1/spaces/AAAAFy1gKzE/messages?key=AIzaSyDdI0hCZtE6vySjMm-WEfRq3CPzqKqqsHI&token=rFHijC_zdQtRNYWsG65G0QvismhSAIL4Z-peRitbR_M%3D".to_string()),
    };

    let serialized_setting = serde_json::to_string(&googleChatConfig).unwrap();

    let path = "google_config.json";

    // let mut output = File::create(path).unwrap();
    //fs::Write(output, serialized_setting.as_str()).unwrap();
    fs::write(path, &serialized_setting).expect("Unable to write file");
}

impl GoogleChatConfig {
    #[tracing::instrument(skip(self))]
    pub fn default_fill(&mut self) {
        let general_mangement = self.general.management.clone();
        let general_employees = self.general.employees.clone();

        //iterate over all groups in group
        for l in self.groups.iter_mut() {
            let local_management = if let Some(value) = &l.management {
                value.clone()
            } else {
                general_mangement.clone()
            };

            l.management = Some(local_management);

            let local_employees = if let Some(value) = &l.employees {
                value.clone()
            } else {
                general_employees.clone()
            };

            l.employees = Some(local_employees);
        }
    }

    #[tracing::instrument]
    pub fn read_from_file(url: String) -> GoogleChatConfig {
        write_struct();
        tracing::info!("wrote sample configuration file");

        let data = fs::read_to_string("configurations/google_chat_config.json")
            .expect("Unable to read file");
        let mut serialised: GoogleChatConfig = serde_json::from_str(data.as_str()).unwrap();
        let chat_url = match serialised.chaturl {
            Some(value) => value,
            None => url,
        };

        serialised.chaturl = Some(chat_url);

        serialised.default_fill();

        serialised
    }
}
