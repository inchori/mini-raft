use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{
    client::RaftClient, node::RaftNode, timer::random_election_timeout, types::{NodeId, Term}
};

pub struct RaftRunner {
    node: Arc<Mutex<RaftNode>>,
    client: RaftClient,
}

impl RaftRunner {
    pub fn new(node: Arc<Mutex<RaftNode>>, peer_addrs: HashMap<NodeId, String>) -> Self {
        Self {
            node,
            client: RaftClient::new(peer_addrs)
        }
    }

    pub async fn tick(&mut self) {
        let (should_start_election, should_send_heartbeat, peers) = {
            let node = self.node.lock().unwrap();
            (
                node.election_timer.is_elapsed() && (node.is_follower() || node.is_candidate()),
                node.heartbeat_timer.is_elapsed() && node.is_leader(),
                node.peers.clone(),
            )
        };

        if should_start_election {
            let (old_term, new_term, node_id, last_log_index, last_log_term) = {
                let mut node = self.node.lock().unwrap();
                let old_term = node.current_term;
                node.become_candidate();
                node.election_timer.reset_with(random_election_timeout());
                (
                    old_term,
                    node.current_term,
                    node.id,
                    node.log.last_log_index(),
                    node.log.last_log_term(),
                )
            };

            println!(
                "  [ELECTION] Node {:?}: Started election (Term {:?} -> {:?})",
                node_id, old_term, new_term
            );

            for peer in &peers {
                let req = crate::raft_proto::RequestVoteRequest {
                    term: new_term.get(),
                    candidate_id: node_id.get(),
                    last_log_index: last_log_index.get(),
                    last_log_term: last_log_term.get(),
                };

                match self.client.send_request_vote(*peer, req).await {
                    Ok(resp) => {
                        let resp = resp.into_inner();
                        let internal_resp = crate::rpc::RequestVoteResponse {
                            term: Term::new(resp.term),
                            vote_granted: resp.vote_granted,
                        };
                        let became_leader = {
                            let mut node = self.node.lock().unwrap();
                            node.handle_request_response(internal_resp)
                        };
                        
                        if became_leader == Some(true) {
                            self.send_heartbeats(&peers).await;
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Failed to send RequestVote to {:?}: {}", peer, e);
                    }
                }
            }
        }

        if should_send_heartbeat {
            {
                let mut node = self.node.lock().unwrap();
                node.heartbeat_timer.reset();
            }
            self.send_heartbeats(&peers).await;
        }
    }

    async fn send_heartbeats(&mut self, peers: &[NodeId]) {
        for peer in peers {
            let req = {
                let node = self.node.lock().unwrap();
                node.create_append_entries(peer)
            };

            match self.client.send_append_entries(*peer, req).await {
                Ok(resp) => {
                    let resp = resp.into_inner();
                    let internal_resp = crate::rpc::AppendEntriesResponse {
                        term: Term::new(resp.term),
                        success: resp.success
                    };
                    let mut node = self.node.lock().unwrap();
                    node.handle_append_entries_response(*peer, internal_resp);
                }   
                Err(e) => {
                    println!("Failed to send AppendEntries to {:?}: {}", peer, e);
                }       
            }
        }
    }
}
