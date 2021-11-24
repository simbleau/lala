use chrono::Utc;
use ringbuffer::{ConstGenericRingBuffer, RingBufferWrite};
use rocket::serde::Serialize;
use rocket_client_addr::ClientAddr;

const HISTORY_SIZE: usize = 2_usize.pow(1);

#[derive(Serialize)]
pub struct HistoryEntry {
    id: i32,
    ip: String,
    kind: String,
    date: String,
}

impl HistoryEntry {
    pub fn new<'a>(id: i32, ip: &'a str, kind: &'a str) -> Self {
        let ip = ip.to_owned();
        let kind = kind.to_owned();
        let date = Utc::now().format("%d%m%Y_%H-%M-%S").to_string();
        HistoryEntry { id, ip, kind, date }
    }
}

pub struct Config {
    on: bool,
    id: i32,
    history: ConstGenericRingBuffer<HistoryEntry, HISTORY_SIZE>,
}

impl Config {
    pub fn status(&self) -> bool {
        self.on
    }
    pub fn signal(&mut self, client_addr: &ClientAddr) {
        // Add history
        self.add_history(HistoryEntry::new(
            self.id,
            &client_addr
                .get_ipv4_string()
                .unwrap_or(client_addr.get_ipv6_string()),
            "signal",
        ));
        // Increment ID
        self.id += 1;
        // Turn off alarm
        self.on = true;
    }
    pub fn silence(&mut self, client_addr: &ClientAddr) {
        // Add history
        self.add_history(HistoryEntry::new(
            self.id,
            &client_addr
                .get_ipv4_string()
                .unwrap_or(client_addr.get_ipv6_string()),
            "silence",
        ));
        // Increment ID
        self.id += 1;
        // Turn off alarm
        self.on = false;
    }
    pub fn get_history(
        &self,
    ) -> &ConstGenericRingBuffer<HistoryEntry, HISTORY_SIZE> {
        &self.history
    }
    pub fn add_history(&mut self, entry: HistoryEntry) {
        self.history.push(entry);
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            on: false,
            id: 0,
            history: ConstGenericRingBuffer::new(),
        }
    }
}
