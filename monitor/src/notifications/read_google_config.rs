use chrono::offset::LocalResult;
use chrono::prelude::*;

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use tracing::info;

#[derive(Deserialize, Serialize, Debug)]
pub struct GoogleChatConfig {
    pub management: Vec<String>,
    pub employees: Vec<String>,
    pub base_url: String, //https://chat.googleapis.com/v1/spaces
    pub token: String, //AIzaSyDdI0hCZtE6vySjMm-WEfRq3CPzqKqqsHI&token=rFHijC_zdQtRNYWsG65G0QvismhSAIL4Z-peRitbR_M%3D
    pub room: String,  // AAAAFy1gKzE
    pub good_msg: String,
    pub error_sev2: String,
    pub error_sev1: String,
}

#[tracing::instrument]
pub fn write_struct() {
    let googleChatConfig = GoogleChatConfig {
        management: vec!["107583083364112988124".to_string()],
        employees: vec!["107583083364112988124".to_string()],
        base_url:"https://chat.googleapis.com/v1/spaces".to_string(),
         token: "AIzaSyDdI0hCZtE6vySjMm-WEfRq3CPzqKqqsHI&token=rFHijC_zdQtRNYWsG65G0QvismhSAIL4Z-peRitbR_M%3D".to_string(), 
        room: "AAAAFy1gKzE".to_string(),
        good_msg: "https://ak.picdn.net/shutterstock/videos/1068883754/thumb/11.jpg".to_string(),
        error_sev2: "https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/160/google/110/heavy-exclamation-mark-symbol_2757.png".to_string(),
        error_sev1: "https://hotemoji.com/images/dl/u/double-exclamation-mark-emoji-by-google.png".to_string(),   
    };

    let serialized_setting = serde_json::to_string(&googleChatConfig).unwrap();

    let path = "google_config.json";

    // let mut output = File::create(path).unwrap();
    //fs::Write(output, serialized_setting.as_str()).unwrap();
    fs::write(path, &serialized_setting).expect("Unable to write file");
}

#[tracing::instrument]
pub fn read_from_file(url: String) -> GoogleChatConfig {
    write_struct();
    tracing::info!("wrote sample configuration file");

    let data =
        fs::read_to_string("configurations/google_chat_config.json").expect("Unable to read file");
    let mut serialised: GoogleChatConfig = serde_json::from_str(data.as_str()).unwrap();

    serialised
}
