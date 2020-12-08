#[path = "udp_handler.rs"]
mod udp_handler;
use serde_json::{Result, Value, Value::String};

use udp_handler::UdpHandler;
pub struct RustyBot {
    socket: UdpHandler,
}

impl RustyBot {
    pub fn new(robot_adrr: &str) -> RustyBot {
        let _socket = UdpHandler::new(robot_adrr).unwrap();
        RustyBot { socket: _socket }
    }

    pub fn start(&mut self) {
        loop {
            let message = self.socket.get_message().unwrap_or("".to_string());
            let val: Result<Value> = serde_json::from_str(message.as_str());
            if val.is_ok() {
                let json = val.unwrap();
                self.parse_json(json);
            } else {
                println!("Not a json!");
            }
        }
    }

    fn parse_json(&mut self, json: Value) {
        match json["cmd"] {
            String() => {
                println!("asasds");
            }
            _ => {
                println!("no command!");
            }
        }
    }
}
