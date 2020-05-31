
/*
  $ ipcal 192.168.1.0/24
  Network addr   : 192.168.1.0
  Hosts addr     : 192.160.1.1 - 192.168.1.254
  Broadcast addr : 192.168.1.255
  Hosts addr num : 254
*/

use std::env;

#[derive(Debug, PartialEq)]
struct IPAddr {
    addr: [u8; 4],
    prefix: u8,
}

impl IPAddr {
    fn compose(&self) -> u32{
        let mut composed_addr :u32 = 0;
        for i in 0..self.addr.len() {
            let octet = (self.addr[i] as u32) << 8 * (self.addr.len() - i - 1);
            composed_addr |= octet;
        }
        composed_addr
    }

    fn generate_network_addr(&self) -> u32 {
        let u32_addr = self.compose();
        u32_addr & ((2u32.pow(self.prefix as u32) - 1) << (32 - self.prefix))
    }

    fn get_humanreadable_addr(&self, ip: u32) -> String {
        let octet_1st = (ip >> 24) & (2u32.pow(8) - 1);
        let octet_2nd = (ip >> 16) & (2u32.pow(8) - 1);
        let octet_3rd = (ip >> 8)  & (2u32.pow(8) - 1);
        let octet_4th = (ip >> 0)  & (2u32.pow(8) - 1);

        format!("{}.{}.{}.{}", octet_1st, octet_2nd, octet_3rd, octet_4th)
    }

    pub fn get_network_addr(&self) -> String {
        let ip = self.generate_network_addr();
        self.get_humanreadable_addr(ip)
    }

    pub fn get_hosts_first_addr(&self) -> String {
        let mut ip = self.generate_network_addr();
        ip += 1;
        self.get_humanreadable_addr(ip)
    }

    pub fn get_hosts_last_addr(&self) -> String {
        let mut ip = self.generate_network_addr();
        ip += 2u32.pow(32 - self.prefix as u32) - 2;
        self.get_humanreadable_addr(ip)
    }

    pub fn get_broadcast_addr(&self) -> String {
        let mut ip = self.generate_network_addr();
        ip += 2u32.pow(32 - self.prefix as u32) - 1;
        self.get_humanreadable_addr(ip)
    }

    pub fn get_hosts_addr_num(&self) -> u32 {
        2u32.pow(32 - self.prefix as u32) - 2
    }
}

fn parse_arg_to_ip(s: &String) -> IPAddr {
    // divide into "addr" and "prefix"
    let ss = &s;
    let separated: Vec<&str> = ss.split('/').collect();
    if separated.len() != 2 {
        eprintln!(
            "error: illegal <ipaddr>"
        );
        std::process::exit(1);
    }

    // get and check prefix
    let prefix: u8 = separated[1].parse().unwrap();
    if prefix > 32 {
        eprintln!(
            "error: prefix length must be less than 32"
        );
        std::process::exit(1);
    }

    // get and check address
    let mut addr: [u8; 4] = Default::default();
    let itr: Vec<&str> = separated[0].split('.').collect();
    if itr.len() != 4 {
        eprintln!(
            "error: address part must be 'x.x.x.x' syntax"
        );
        std::process::exit(1);
    }
    for i in 0..itr.len() {
        // for catching error
        let a: u32 = itr[i].parse().unwrap_or(256);
        if a > 255 {
            eprintln!(
                "error: each octets must be less than 255"
            );
            std::process::exit(1);
        }
        addr[i] = a as u8;
    }

    IPAddr {
        addr: addr,
        prefix: prefix,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Usage: ipcal <ipaddr>"
        );
        std::process::exit(1);
    }

    let ip = parse_arg_to_ip(&args[1]);
    println!("Network addr   : {}", ip.get_network_addr());
    println!("Hosts addr     : {} - {}", ip.get_hosts_first_addr(), ip.get_hosts_last_addr());
    println!("Broadcast addr : {}", ip.get_broadcast_addr());
    println!("Hosts addr num : {}", ip.get_hosts_addr_num());
}

#[cfg(test)]
mod tests {
    use super::IPAddr;

    #[test]
    fn parse_string_to_ip() {
        let s1 = "192.168.1.0/24";
        let ip1 = IPAddr { addr: [192, 168, 1, 0], prefix: 24 };
        assert_eq!(super::parse_arg_to_ip(&s1.to_string()), ip1);

        let s2 = "10.0.1.0/20";
        let ip2 = IPAddr { addr: [10, 0, 1, 0], prefix: 20 };
        assert_eq!(super::parse_arg_to_ip(&s2.to_string()), ip2);
    }

    #[test]
    fn generate_ip_string() {
        let ip1 = IPAddr {
            addr: [192, 168, 1, 0],
            prefix: 24,
        };
        assert_eq!(ip1.get_network_addr(), "192.168.1.0");
        assert_eq!(ip1.get_hosts_first_addr(), "192.168.1.1");
        assert_eq!(ip1.get_hosts_last_addr(), "192.168.1.254");
        assert_eq!(ip1.get_hosts_addr_num(), 254);

        let ip2 = IPAddr {
            addr: [10, 0, 40, 1],
            prefix: 19,
        };
        assert_eq!(ip2.get_network_addr(), "10.0.32.0");
        assert_eq!(ip2.get_hosts_first_addr(), "10.0.32.1");
        assert_eq!(ip2.get_hosts_last_addr(), "10.0.63.254");
        assert_eq!(ip2.get_hosts_addr_num(), 8190);
    }
}