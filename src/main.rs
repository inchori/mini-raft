use mini_raft::{log::{LogEntry, LogStore}, types::{LogIndex, Term}};

fn main() {
    let mut store = LogStore::new();
    
    let entry1 = LogEntry {
        term: Term::new(1),
        index: LogIndex::new(1),
        command: vec![1, 2, 3],
    };
    
    store.append(entry1);
    println!("Last log index: {:?}", store.last_log_index());
}
