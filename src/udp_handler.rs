use std::net::UdpSocket;

#[derive(Debug)]
pub struct UdpHandler {
    socket: UdpSocket,
    server_addr: String,
}

impl UdpHandler {
    pub fn new(addr: &str) -> Result<UdpHandler, std::io::Error> {
        let mut _socket = UdpSocket::bind("127.0.0.1:3456")?;
        Ok(UdpHandler {
            socket: _socket,
            server_addr: addr.to_string(),
        })
    }
    pub fn get_message(&mut self) -> Result<String, std::io::Error> {
        let mut buf = [0; 256];
        let (amt, _src) = self.socket.recv_from(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf[..amt]).to_string())
    }
}
