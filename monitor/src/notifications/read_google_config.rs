use serde::{Deserialize, Serialize};
use std::fs;

use std::io::{BufRead, BufReader, Error, Write};
use tracing::info;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GoogleChatConfig {
    pub management: Vec<String>,
    pub employees: Vec<String>,
    pub chat_url: String,
    pub good_msg: String,
    pub error_sev2: String,
    pub error_sev1: String,
}

#[tracing::instrument]
pub fn write_struct() {
    let googleChatConfig = GoogleChatConfig {
        management: vec!["107583083364112988124".to_string()],
        employees: vec!["107583083364112988124".to_string()],
        chat_url:"https://chat.googleapis.com/v1/spaces/AAAAFy1gKzE/messages?key=AIzaSyDdI0hCZtE6vySjMm-WEfRq3CPzqKqqsHI&token=rFHijC_zdQtRNYWsG65G0QvismhSAIL4Z-peRitbR_M%3D".to_string(), 
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

impl GoogleChatConfig {
    #[tracing::instrument]
    pub fn read_from_file() -> GoogleChatConfig {
        //write_struct(); // for making json quickly
        //tracing::info!("wrote sample configuration file");

        let data: String;
        let res = fs::read_to_string("configurations/google_chat_config.json");

        match res {
            Ok(value) => data = value.to_string(),
            Err(err) => {
                tracing::error!(
                    "Error occured while reading google configuratioon file => {}",
                    &err
                );
                panic!(
                    "Error occured while reading google configuratioon file => {}",
                    &err
                );
            }
        }

        tracing::info!("Google configuration file succesfullt read");

        let mut serialised: GoogleChatConfig = serde_json::from_str(data.as_str()).unwrap();

        serialised
    }
}
