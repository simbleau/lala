use chrono::Utc;
use rocket_client_addr::ClientAddr;
use server_util::data_structures::History;
use server_util::models::AlarmStatus;
use server_util::models::HistoryKind;

pub struct AlarmState {
    status: AlarmStatus,
    history: History,
}

impl AlarmState {
    pub fn status(&self) -> &AlarmStatus {
        &self.status
    }
    pub fn signal(&mut self, client_addr: &ClientAddr) {
        let addr = server_util::addr_to_string(client_addr);
        let date = Utc::now(); // TODO replace with request time

        // Add history log
        self.history.add(addr, date, HistoryKind::Signal);
        // Turn on alarm
        self.status = AlarmStatus::On;
    }
    pub fn silence(&mut self, client_addr: &ClientAddr) {
        let addr = server_util::addr_to_string(client_addr);
        let date = Utc::now(); // TODO replace with request time

        // Add history log
        self.history.add(addr, date, HistoryKind::Silence);
        // Turn off alarm
        self.status = AlarmStatus::Off;
    }
    pub fn history(&self) -> &History {
        &self.history
    }
}

impl Default for AlarmState {
    fn default() -> Self {
        AlarmState {
            status: AlarmStatus::Off,
            history: History::new(),
        }
    }
}
