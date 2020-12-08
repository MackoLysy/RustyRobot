mod rusty_bot;
use rusty_bot::RustyBot;
fn main() {
    let mut robot = RustyBot::new("127.0.0.1:3456");
    robot.start();
}
// let mut socket = UdpSocket::bind("127.0.0.1:34254")?;

// // Receives a single datagram message on the socket. If `buf` is too small to hold
// // the message, it will be cut off.
// loop {
//     let mut buf = [0; 10];
//     let (amt, src) = socket.recv_from(&mut buf)?;
//     // Redeclare `buf` as slice of the received data and send reverse data back to origin.
//     let buf = &mut buf[..amt];
//     println!("-> {}\n", String::from_utf8_lossy(&buf[..amt]));
//     buf.reverse();
//     socket.send_to(buf, &src)?;
// }
