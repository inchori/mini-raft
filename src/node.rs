use std::collections::HashMap;

use crate::log::LogStore;
use crate::types::{LogIndex, NodeId, RaftState, Term};

#[derive(Debug, Clone)]
pub struct RaftNode {
    pub id: NodeId,
    pub peers: Vec<NodeId>,

    pub state: RaftState,

    // Persistent State
    pub current_term: Term,
    pub voted_for: Option<NodeId>,
    pub log: LogStore,

    // Volatile State
    pub commit_index: LogIndex,
    pub last_applied: LogIndex,

    // Leader State
    pub next_index: HashMap<NodeId, LogIndex>,
    pub match_index: HashMap<NodeId, LogIndex>
}

impl RaftNode {
    pub fn new(id: NodeId, peers: Vec<NodeId>) -> Self {
        Self {
            id,
            peers,
            state: RaftState::Follower,
            current_term: Term::ZERO,
            voted_for: None,
            log: LogStore::new(),
            commit_index: LogIndex::ZERO,
            last_applied: LogIndex::ZERO,
            next_index: HashMap::new(),
            match_index: HashMap::new(),
        }
    }

    pub fn is_leader(&self) -> bool {
        self.state == RaftState::Leader
    }

    pub fn is_follower(&self) -> bool {
        self.state == RaftState::Follower
    }

    pub fn is_candidate(&self) -> bool {
        self.state == RaftState::Candidate
    }

    pub fn cluster_size(&self) -> usize {
        self.peers.len() + 1
    }

    pub fn quorum(&self) -> usize {
        self.cluster_size() / 2 + 1
    }
}