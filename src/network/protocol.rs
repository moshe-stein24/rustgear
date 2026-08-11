use std::net::SocketAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    UDP,
    TCP,
}

#[derive(Debug, Clone)]
pub struct SocketAddress {
    pub host: String,
    pub port: u16,
    pub protocol: Protocol,
}

impl SocketAddress {
    pub fn new(host: &str, port: u16, protocol: Protocol) -> Self {
        Self { host: host.to_string(), port, protocol }
    }

    pub fn to_socket_addr(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        format!("{}:{}", self.host, self.port).parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn socket_addr_udp() {
        let addr = SocketAddress::new("127.0.0.1", 5010, Protocol::UDP);
        let parsed = addr.to_socket_addr().unwrap();
        assert_eq!(parsed.port(), 5010);
    }
}
