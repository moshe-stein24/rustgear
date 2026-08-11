use std::net::UdpSocket;

#[derive(Debug)]
pub struct UdpTransport {
    pub socket: UdpSocket,
    pub peer: Option<std::net::SocketAddr>,
}

impl UdpTransport {
    pub fn bind(addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;
        Ok(Self { socket, peer: None })
    }

    pub fn send(&mut self, payload: &[u8]) -> Result<usize, std::io::Error> {
        let addr = self.peer.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotConnected, "no peer"))?;
        self.socket.send_to(payload, addr)
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<(usize, std::net::SocketAddr), std::io::Error> {
        self.socket.recv_from(buf)
    }

    pub fn connect(&mut self, peer: std::net::SocketAddr) -> Result<(), std::io::Error> {
        self.peer = Some(peer);
        self.socket.connect(peer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn udp_bind_localhost() {
        let _t = UdpTransport::bind("127.0.0.1:0").expect("bind UDP");
    }
}
