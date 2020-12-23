mod rusty_bot;
mod udp_handler;
use rusty_bot::RustyBot;
use udp_handler::UdpHandler;

fn main() {
    let mut robot = RustyBot::new();
    let mut udp = UdpHandler::new("127.0.0.1:34254").unwrap();
    loop {
        let message = udp.get_message();
        if message.is_ok() {
            robot.get_message(message.unwrap());
        } else {
            println!("Not a message!");
        }
    }
}
