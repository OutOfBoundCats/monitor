use pinger::{ping, PingResult};

pub fn pin_host(host: String) -> bool {
    let mut ping_response: bool = false;
    let stream = ping(host).expect("Error pinging");
    for message in stream {
        match message {
            PingResult::Pong(duration, _) => {
                println!("{:?}", duration);
                ping_response = true;
            }
            PingResult::Timeout(_) => {
                println!("Timeout!");
                ping_response = false;
            }
            PingResult::Unknown(line) => (),
        }
    }

    ping_response
}
