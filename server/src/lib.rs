pub mod data_structures;
pub mod models;

mod cors;
pub use cors::CORS;

pub const AUTHORITY_PORT: u16 = 8081;
pub const ALARM_PORT: u16 = 8082;

pub fn addr_to_string(addr: &rocket_client_addr::ClientAddr) -> String {
    if addr.ip.is_loopback() {
        "0.0.0.0".to_string()
    } else {
        addr.get_ipv4_string().unwrap_or(addr.get_ipv6_string())
    }
}
