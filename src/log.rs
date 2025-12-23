use crate::types::{LogIndex, Term};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub term: Term,
    pub index: LogIndex,
    pub command: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct LogStore {
    pub entries: Vec<LogEntry>,
}

impl LogStore {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn append(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }

    // TODO: O(n) -> O(1)
    pub fn get(&self, index: LogIndex) -> Option<&LogEntry> {
        self.entries.iter().find(|&i| i.index == index)
    }

    pub fn last_log_index(&self) -> LogIndex {
        if self.entries.is_empty() {
            LogIndex::ZERO
        } else {
            self.entries.last().unwrap().index
        }
    }

    pub fn last_log_term(&self) -> Term {
        if self.entries.is_empty() {
            Term::ZERO
        } else {
            self.entries.last().unwrap().term
        }
    }

    pub fn entries_from(&self, from_index: LogIndex) -> Vec<LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.index >= from_index)
            .cloned()
            .collect()
    }

    pub fn truncate(&mut self, from_index: LogIndex) {
        self.entries.retain(|entry| entry.index < from_index);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}