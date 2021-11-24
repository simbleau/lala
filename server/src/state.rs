use chrono::Utc;
use ringbuffer::{ConstGenericRingBuffer, RingBufferWrite};
use rocket::serde::Serialize;
use rocket_client_addr::ClientAddr;

pub struct AlarmState {
    on: bool,
    history: History,
}

impl AlarmState {
    pub fn status(&self) -> bool {
        self.on
    }
    pub fn signal(&mut self, client_addr: &ClientAddr) {
        // Add history log
        self.history.add(
            &client_addr
                .get_ipv4_string()
                .unwrap_or(client_addr.get_ipv6_string()),
            "signal",
        );
        // Turn on alarm
        self.on = true;
    }
    pub fn silence(&mut self, client_addr: &ClientAddr) {
        // Add history log
        self.history.add(
            &client_addr
                .get_ipv4_string()
                .unwrap_or(client_addr.get_ipv6_string()),
            "silence",
        );
        // Turn off alarm
        self.on = false;
    }
    pub fn history(&self) -> &History {
        &self.history
    }
}

impl Default for AlarmState {
    fn default() -> Self {
        AlarmState {
            on: false,
            history: History::new(),
        }
    }
}

const HISTORY_SIZE: usize = 2_usize.pow(10);

pub struct History {
    next_id: i32,
    history: ConstGenericRingBuffer<HistoryEntry, HISTORY_SIZE>,
}

impl History {
    fn new() -> Self {
        History {
            next_id: 0,
            history: ConstGenericRingBuffer::new(),
        }
    }

    pub fn entries(
        &self,
    ) -> &ConstGenericRingBuffer<HistoryEntry, HISTORY_SIZE> {
        &self.history
    }

    pub fn add<'a>(&mut self, ip: &'a str, kind: &'a str) {
        let id = self.next_id;
        self.next_id += 1;
        let ip = ip.to_owned();
        let kind = kind.to_owned();
        let date = Utc::now().format("%d%m%Y_%H-%M-%S").to_string();
        self.history.push(HistoryEntry { id, ip, kind, date });
    }
}

#[derive(Serialize)]
pub struct HistoryEntry {
    id: i32,
    ip: String,
    kind: String,
    date: String,
}
