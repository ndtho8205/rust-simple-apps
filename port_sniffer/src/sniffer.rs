use std::net::{IpAddr, TcpStream};

pub fn scan(ip: IpAddr, port: u16) -> bool {
    match TcpStream::connect((ip, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn scan_localhost_failure() {
        assert_eq!(scan(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1), false)
    }
}
