use chrono::{DateTime, Duration, Utc};
use priority_queue::DoublePriorityQueue;
use rocket_client_addr::ClientAddr;
use server_util::{
    data_structures::History,
    models::{HistoryKind, Host},
};
use std::ops::Sub;

pub struct ServerState {
    alarms: DoublePriorityQueue<String, DateTime<Utc>>,
    history: History,
}

impl ServerState {
    fn expiration() -> DateTime<Utc> {
        Utc::now().sub(Duration::minutes(15))
    }

    pub fn prune(&mut self) {
        // TODO This isn't what we want.
        // We want something O(logn) like `queue.clear_over(priority)`

        // Collect all elements over the expiration time O(n)
        let expiration = &ServerState::expiration();
        let mut expired_addrs = Vec::new();
        for (addr, time) in self.alarms.iter() {
            if time.lt(expiration) {
                expired_addrs.push(addr.clone());
            }
        }
        // Prune
        for expired_addr in expired_addrs {
            self.alarms.remove(&expired_addr);
        }
    }
    pub fn alarms(&self) -> Vec<Host> {
        let expiration = &ServerState::expiration();
        let mut hosts = Vec::new();

        // TODO This isn't what we want.
        // We want something O(logn) like `queue.items_over(priority)`
        for (addr, time) in self.alarms.iter() {
            if time.ge(expiration) {
                hosts.push(Host {
                    addr: addr.to_owned(),
                    last_checkin: time.to_owned(),
                })
            }
        }
        hosts
    }
    pub fn checkin(&mut self, client_addr: &ClientAddr) {
        let addr = server_util::addr_to_string(client_addr);
        let date = Utc::now(); // TODO replace with request time

        // Add history log
        self.history
            .add(addr.clone(), date, HistoryKind::AlarmCheckin);
        // Insert address
        self.alarms.push_increase(addr, date);
    }
}

impl Default for ServerState {
    fn default() -> Self {
        ServerState {
            alarms: DoublePriorityQueue::<String, chrono::DateTime<Utc>>::new(),
            history: History::new(),
        }
    }
}
