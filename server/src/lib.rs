pub mod data_structures;
pub mod models;

pub fn addr_to_string(addr: &rocket_client_addr::ClientAddr) -> String {
    addr.get_ipv4_string().unwrap_or(addr.get_ipv6_string())
}
