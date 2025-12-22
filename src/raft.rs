use std::collections::VecDeque;

use crate::{
    event::RaftEvent,
    node::RaftNode,
    rpc::{AppendEntriesRequest, RequestVoteRequest},
    timer::random_election_timeout,
    types::NodeId,
};

pub struct RaftRunner {
    node: RaftNode,
    event_queue: VecDeque<RaftEvent>,
}

#[derive(Debug, Clone)]
pub enum RaftAction {
    SendRequestVote(NodeId, RequestVoteRequest),
    SendAppendEntries(NodeId, AppendEntriesRequest),
}

impl RaftRunner {
    pub fn new(node: RaftNode) -> Self {
        Self {
            node,
            event_queue: VecDeque::new(),
        }
    }

    pub fn node_mut(&mut self) -> &mut RaftNode {
        &mut self.node
    }

    pub fn tick(&mut self) -> Vec<RaftAction> {
        let mut actions = Vec::new();

        if self.node.election_timer.is_elapsed() {
            if self.node.is_follower() || self.node.is_candidate() {
                let old_term = self.node.current_term;
                self.node.become_candidate();
                self.node
                    .election_timer
                    .reset_with(random_election_timeout());

                println!(
                    "  🗳️  Node {:?}: Started election (Term {:?} → {:?})",
                    self.node.id, old_term, self.node.current_term
                );

                for peer in &self.node.peers.clone() {
                    let request = RequestVoteRequest {
                        term: self.node.current_term,
                        candidate_id: self.node.id,
                        last_log_index: self.node.log.last_log_index(),
                        last_log_term: self.node.log.last_log_term(),
                    };
                    actions.push(RaftAction::SendRequestVote(*peer, request));
                }
            }
        }

        if self.node.heartbeat_timer.is_elapsed() {
            if self.node.is_leader() {
                self.node.heartbeat_timer.reset();

                for peer in &self.node.peers.clone() {
                    let request = self.node.create_append_entries(peer);
                    actions.push(RaftAction::SendAppendEntries(*peer, request));
                }
            }
        }

        while let Some(event) = self.event_queue.pop_front() {
            self.handle_event(event, &mut actions);
        }

        actions
    }

    pub fn push_event(&mut self, event: RaftEvent) {
        self.event_queue.push_back(event);
    }

    pub fn handle_event(&mut self, event: RaftEvent, actions: &mut Vec<RaftAction>) {
        match event {
            RaftEvent::ReceivedRequestVote(request) => {
                let _response = self.node.handle_request_vote(request);
            }
            RaftEvent::ReceivedRequestVoteResponse(response) => {
                self.node.handle_request_response(response);
            }
            RaftEvent::ReceivedAppendEntries(request) => {
                let _response = self.node.handle_append_entries(request);
                self.node
                    .election_timer
                    .reset_with(random_election_timeout());
            }
            RaftEvent::ReceivedAppendEntriesResponse(response) => {}

            _ => {}
        }
    }

    pub fn node(&self) -> &RaftNode {
        &self.node
    }
}
