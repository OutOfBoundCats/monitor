use std::fs;

use crate::{monitors::LocalItems, notifications::read_google_config::GoogleChatConfig};

pub struct Response {
    pub response_msg: String,
    pub response_code: u16,
}

impl GoogleChatConfig {
    #[tracing::instrument(skip(self, msg))]
    pub fn send_chat_msg(&self, msg: String) {
        let json_string: String = msg;

        let g_url = self.chaturl.as_ref().unwrap();

        let client = reqwest::blocking::Client::new();
        let res = client.post(g_url).body(json_string.clone()).send().unwrap();

        let status_code = res.status().as_u16();
        let responsetxt = res.text_with_charset("UTF-8").unwrap().clone();

        let response = Response {
            response_msg: responsetxt,
            response_code: status_code.clone(),
        };
        tracing::info!("json paylaod sent  is {}", &json_string);
        tracing::info!("respose recived is {}", &response.response_msg);
    }

    #[tracing::instrument(skip(self, item, status, severity, msg))]
    pub fn build_msg(&self, item: &LocalItems, status: &str, severity: i32, msg: String) -> String {
        let mut data = fs::read_to_string("messages/google_msg.json").expect("Unable to read file");

        tracing::info!("file data read is {}", &data);

        let mut image_url = "".to_string();

        //Header 1
        let mut header1;
        if status == "ERROR" {
            header1 = &item.message[0];
            if severity == 2 {
                image_url = self.general.error_sev2.clone();
            } else if severity == 1 {
                image_url = self.general.error_sev1.clone();
            }
        } else {
            header1 = &item.message[1];
            image_url = self.general.good_msg.clone();
        }

        let new_header1 = &header1.replacen("{TXT}", &item.label, 1);

        //text 1
        data = data.replacen("{TXT}", &new_header1, 1);

        //Header 2
        let mut header2 = "".to_string();
        let mut temp = "<users/{}> ";
        //IF item name is CPU put CPU list in header mesage 2
        if item.name == "CPU" {
            for group in &self.groups {
                if group.name == "CPU" {
                    if severity == 2 {
                        for employees in &group.employees {
                            for employee in employees {
                                let temp2 = temp.replacen("{}", &employee, 1);
                                header2.push_str(&temp2);
                            }
                        }
                    }
                }
            }
        } else {
        }

        //text 2
        data = data.replacen("{TXT}", &header2, 1);

        //header title
        data = data.replacen("{TXT}", &new_header1, 1);

        //header subtitle
        data = data.replacen("{TXT}", &msg, 1);

        //header imageUrl
        data = data.replacen("{TXT}", &image_url, 1);

        // sections widgets textParagraph text
        data = data.replacen("{TXT}", &msg, 1);

        tracing::info!("json created is {}", &data);
        data
    }
}
