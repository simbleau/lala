pub mod data_structures;
pub mod models;

pub const AUTHORITY_PORT: u16 = 8081;
pub const ALARM_PORT: u16 = 8082;

pub fn addr_to_string(addr: &rocket_client_addr::ClientAddr) -> String {
    addr.get_ipv4_string().unwrap_or(addr.get_ipv6_string())
}
