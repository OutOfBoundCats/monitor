use crate::notifications::read_google_config::GoogleChatConfig;

pub struct Response {
    pub response_msg: String,
    pub response_code: u16,
}

impl GoogleChatConfig {
    pub fn send_chat_msg(self, url: String, msg: String) -> Response {
        let json_string: String = msg;

        let client = reqwest::blocking::Client::new();
        let res = client.post(url).body(json_string).send().unwrap();

        let status_code = res.status().as_u16();
        let responsetxt = res.text_with_charset("UTF-8").unwrap().clone();

        let response = Response {
            response_msg: responsetxt,
            response_code: status_code.clone(),
        };

        response
    }
}
