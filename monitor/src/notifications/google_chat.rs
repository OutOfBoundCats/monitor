use crate::notifications::read_google_config::GoogleChatConfig;

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
}
