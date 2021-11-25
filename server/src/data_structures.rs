use chrono::{DateTime, Utc};
use ringbuffer::{ConstGenericRingBuffer, RingBufferWrite};

use crate::models::{HistoryEntry, HistoryKind};

const HISTORY_SIZE: usize = 2_usize.pow(10);

pub struct History {
    next_id: i32,
    history: ConstGenericRingBuffer<HistoryEntry, HISTORY_SIZE>,
}

impl History {
    pub fn new() -> Self {
        History {
            next_id: 0,
            history: ConstGenericRingBuffer::<_, HISTORY_SIZE>::new(),
        }
    }

    pub fn entries(
        &self,
    ) -> &ConstGenericRingBuffer<HistoryEntry, HISTORY_SIZE> {
        &self.history
    }

    pub fn add(
        &mut self,
        addr: String,
        date: DateTime<Utc>,
        kind: HistoryKind,
    ) {
        let id = self.next_id;
        self.next_id += 1;
        self.history.push(HistoryEntry {
            id,
            addr,
            kind,
            date,
        });
    }
}
