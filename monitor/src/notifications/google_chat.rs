use std::{convert::TryInto, fs};

use crate::notifications::read_google_config::GoogleChatConfig;

pub struct Response {
    pub response_msg: String,
    pub response_code: u16,
}

impl GoogleChatConfig {
    #[tracing::instrument(skip(self, msg))]
    pub fn send_chat_msg(&self, msg: String) {
        let json_string: String = msg;

        let g_url = &self.chat_url;

        let client = reqwest::blocking::Client::new();
        let res = client.post(g_url).body(json_string.clone()).send().unwrap();
        tracing::info!("sent payload is {}", &json_string);

        let status_code = res.status().as_u16();
        let responsetxt = res.text_with_charset("UTF-8").unwrap().clone();

        let response = Response {
            response_msg: responsetxt,
            response_code: status_code.clone(),
        };
        //tracing::info!("json paylaod sent  is {}", &json_string);
        //tracing::info!("respose recived is {}", &response.response_msg);
    }

    #[tracing::instrument(skip(self, severity, message, priority, label, target))]
    pub fn build_msg(
        &self,
        severity: i32,
        message: &String,
        priority: i32,
        label: &String,
        target: &String,
    ) -> String {
        let mut data = fs::read_to_string("messages/google_msg.json").expect("Unable to read file");

        let mut image_url = "".to_string();

        if severity == 2 {
            image_url = self.error_sev2.clone();
        } else if severity == 1 {
            image_url = self.error_sev1.clone();
        }

        //Header 1
        let mut users = "".to_string();
        let mut temp = "<users/{{}}> ";

        // include user id in header text
        if severity == 2 {
            for employees in &self.employees {
                temp = "<users/{{}}> ";
                let temp2 = temp.replacen("{{}}", &employees, 1);
                users.push_str(&temp2);
            }
        } else if severity == 1 {
            for managers in &self.management {
                temp = "<users/{{}}> ";
                let temp2 = temp.replacen("{{}}", &managers, 1);
                users.push_str(&temp2);
            }
        }

        let l_priority = priority;
        let mut message = message;

        //1 . user text
        data = data.replacen("{{}}", &users, 1);

        //2 .header title
        data = data.replacen("{{}}", &message, 1);

        //3 .header subtitle
        let priority_str = format!("Priority is {}", &l_priority);
        data = data.replacen("{{}}", &priority_str, 1);

        //4 .header imageUrl
        data = data.replacen("{{}}", &image_url, 1);

        //5 .section textparagraph text
        data = data.replacen(
            "{{}}",
            format!("{} with priority {}", &message, &l_priority).as_str(),
            1,
        );

        //tracing::info!("json created is {}", &data);
        data
    }
}
