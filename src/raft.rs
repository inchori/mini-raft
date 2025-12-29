use std::collections::{HashMap, VecDeque};

use crate::{
    client::RaftClient, event::RaftEvent, node::RaftNode, timer::random_election_timeout, types::{NodeId, Term}
};

pub struct RaftRunner {
    node: RaftNode,
    client: RaftClient,
    event_queue: VecDeque<RaftEvent>,
}

impl RaftRunner {
    pub fn new(node: RaftNode, peer_addrs: HashMap<NodeId, String>) -> Self {
        Self {
            node,
            event_queue: VecDeque::new(),
            client: RaftClient::new(peer_addrs)
        }
    }

    pub fn node_mut(&mut self) -> &mut RaftNode {
        &mut self.node
    }

    pub async fn tick(&mut self) {
        if self.node.election_timer.is_elapsed() {
            if self.node.is_follower() || self.node.is_candidate() {
                let old_term = self.node.current_term;
                self.node.become_candidate();
                self.node
                    .election_timer
                    .reset_with(random_election_timeout());

                println!(
                    "  [ELECTION] Node {:?}: Started election (Term {:?} -> {:?})",
                    self.node.id, old_term, self.node.current_term
                );

                for peer in &self.node.peers.clone() {
                    //TODO: change to From trait in rpc.rs
                    let req = crate::raft_proto::RequestVoteRequest {
                        term: self.node.current_term.get(),
                        candidate_id: self.node.id.get(),
                        last_log_index: self.node.log.last_log_index().get(),
                        last_log_term: self.node.log.last_log_term().get(),
                    };

                    match self.client.send_request_vote(*peer, req).await {
                        Ok(resp) => {
                            let resp = resp.into_inner();
                            let internal_resp = crate::rpc::RequestVoteResponse {
                                term: Term::new(resp.term),
                                vote_granted: resp.vote_granted,
                            };
                            self.node.handle_request_response(internal_resp);
                        }
                        Err(e) => {
                            println!("Failed to send RequestVote to {:?}: {}", peer, e);
                        }
                    }
                }
            }
        }

        if self.node.heartbeat_timer.is_elapsed() {
            if self.node.is_leader() {
                self.node.heartbeat_timer.reset();

                for peer in &self.node.peers.clone() {
                    let req = self.node.create_append_entries(peer);

                    match self.client.send_append_entries(*peer, req).await {
                        Ok(resp) => {
                            let resp = resp.into_inner();
                            let internal_resp = crate::rpc::AppendEntriesResponse {
                                term: Term::new(resp.term),
                                success: resp.success
                            };
                            self.node.handle_append_entries_response(*peer, internal_resp);
                        }   
                        Err(e) => {
                            println!("Failed to send AppendEntries to {:?}: {}", peer, e);
                        }       
                    }
                }
            }
        }

    }

    pub fn push_event(&mut self, event: RaftEvent) {
        self.event_queue.push_back(event);
    }

    pub fn handle_event(&mut self, event: RaftEvent) {
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
            RaftEvent::ReceivedAppendEntriesResponse(_response) => {}
            
            _ => {}
        }
    }

    pub fn node(&self) -> &RaftNode {
        &self.node
    }
}
