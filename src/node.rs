use std::collections::HashMap;

use crate::log::LogStore;
use crate::rpc::{
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};
use crate::timer::{Timer, heartbeat_interval, random_election_timeout};
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
    pub match_index: HashMap<NodeId, LogIndex>,

    // Timer
    pub election_timer: Timer,
    pub heartbeat_timer: Timer,
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
            election_timer: Timer::new(random_election_timeout()),
            heartbeat_timer: Timer::new(heartbeat_interval()),
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

    pub fn become_follower(&mut self, term: Term) {
        self.state = RaftState::Follower;
        self.current_term = term;
        self.voted_for = None;
    }

    pub fn become_candidate(&mut self) {
        self.state = RaftState::Candidate;
        self.current_term = Term::new(self.current_term.get() + 1);
        self.voted_for = Some(self.id);
    }

    pub fn become_leader(&mut self) {
        self.state = RaftState::Leader;

        let next_index_value = LogIndex::new(self.log.last_log_index().get() + 1);

        for peer in &self.peers {
            self.next_index.insert(*peer, next_index_value);
            self.match_index.insert(*peer, LogIndex::ZERO);
        }
    }

    pub fn handle_request_vote(&mut self, request: RequestVoteRequest) -> RequestVoteResponse {
        if request.term > self.current_term {
            self.become_follower(request.term);
        }

        let can_vote = request.term >= self.current_term
            && (self.voted_for.is_none() || self.voted_for == Some(request.candidate_id))
            && self.is_log_up_to_date(request.last_log_term, request.last_log_index);

        if can_vote {
            self.voted_for = Some(request.candidate_id);
        }

        RequestVoteResponse {
            term: self.current_term,
            vote_granted: can_vote,
        }
    }

    pub fn handle_request_response(&mut self, response: RequestVoteResponse) -> Option<bool> {
        if !self.is_candidate() {
            return None;
        }

        if response.term > self.current_term {
            self.become_follower(response.term);
            return Some(false);
        }

        if response.vote_granted {
            None
        } else {
            None
        }
    }

    pub fn is_log_up_to_date(
        &self,
        candidate_last_log_term: Term,
        candidate_last_log_index: LogIndex,
    ) -> bool {
        let my_last_term = self.log.last_log_term();
        let my_last_index = self.log.last_log_index();

        if candidate_last_log_term > my_last_term {
            return true;
        }

        if candidate_last_log_term == my_last_term {
            return candidate_last_log_index >= my_last_index;
        }

        false
    }

    pub fn handle_append_entries(
        &mut self,
        request: AppendEntriesRequest,
    ) -> AppendEntriesResponse {
        if request.term > self.current_term {
            self.become_follower(request.term);
        }

        if request.term < self.current_term {
            return AppendEntriesResponse {
                term: self.current_term,
                success: false,
            };
        }

        AppendEntriesResponse {
            term: self.current_term,
            success: true,
        }
    }

    pub fn create_append_entries(&self, peer: &NodeId) -> AppendEntriesRequest {
        let next_idx = self
            .next_index
            .get(peer)
            .copied()
            .unwrap_or(LogIndex::new(1));

        let prev_log_index = LogIndex::new(next_idx.get().saturating_sub(1));
        let prev_log_term = if prev_log_index.get() == 0 {
            Term::ZERO
        } else {
            self.log
                .get(prev_log_index)
                .map(|e| e.term)
                .unwrap_or(Term::ZERO)
        };

        let entries = vec![];

        AppendEntriesRequest {
            term: self.current_term,
            leader_id: self.id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit: self.commit_index,
        }
    }

    pub fn handle_append_entries_response(
        &mut self,
        peer: NodeId,
        response: AppendEntriesResponse,
    ) {
        if !self.is_leader() {
            return;
        }

        if response.term > self.current_term {
            self.become_follower(response.term);
            return;
        }

        if response.success {
            // TODO: update match index
        } else {
            if let Some(next_idx) = self.next_index.get(&peer).copied() {
                if next_idx.get() > 1 {
                    self.next_index
                        .insert(peer, LogIndex::new(next_idx.get() - 1));
                }
            }
        }
    }
}
