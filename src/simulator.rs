use std::collections::HashMap;

use crate::{event::RaftEvent, node::RaftNode, raft::{RaftAction, RaftRunner}, types::NodeId};

pub struct Simulator {
    runners: HashMap<NodeId, RaftRunner>,
    vote_counts: HashMap<NodeId, usize>,
}

impl Simulator {
    pub fn new(node_ids: Vec<NodeId>) -> Self {
        let mut runners = HashMap::new();

        for &id in &node_ids {
            let peers: Vec<NodeId> = node_ids
                .iter()
                .copied()
                .filter(|&peer_id| peer_id != id)
                .collect();

            let node = RaftNode::new(id, peers);
            let runner = RaftRunner::new(node);
            runners.insert(id, runner);
        }

        Self {
            runners,
            vote_counts: HashMap::new(),
        }
    }

    pub fn tick(&mut self) {
        let mut all_actions: Vec<(NodeId, RaftAction)> = Vec::new();

        for (&id, runner) in &mut self.runners {
            let before_state = runner.node().state;

            let actions = runner.tick();

            let after_state = runner.node().state;

            if before_state != after_state {
                println!("  [STATE] Node {:?}: {:?} -> {:?}", id, before_state, after_state);
            }

            for action in actions {
                all_actions.push((id, action));
            }
        }

        for (from, action) in all_actions {
            self.handle_action(from, action);
        }
    }

    fn handle_action(&mut self, from: NodeId, action: RaftAction) {
        match action {
            RaftAction::SendRequestVote(to, request) => {
                if let Some(target_runner) = self.runners.get_mut(&to) {
                    target_runner.push_event(RaftEvent::ReceivedRequestVote(request.clone()));

                    let response = target_runner.node_mut().handle_request_vote(request);

                    if response.vote_granted {
                        let count = self.vote_counts.entry(from).or_insert(1);
                        *count += 1;

                        let quorum = self.runners.get(&from).unwrap().node().quorum();
                        let is_candidate = self.runners.get(&from).unwrap().node().is_candidate();
                        
                        if *count >= quorum && is_candidate {
                            let sender_runner = self.runners.get_mut(&from).unwrap();
                            sender_runner.node_mut().become_leader();
                            println!("[LEADER] Node {:?} became Leader! (votes: {})", from, *count);
                            
                            self.send_heartbeats(from);
                            
                            // vote_counts 초기화
                            self.vote_counts.remove(&from);
                        }
                    }
                }
            }
            
            RaftAction::SendAppendEntries(to, request) => {
                if let Some(target_runner) = self.runners.get_mut(&to) {
                    target_runner.push_event(RaftEvent::ReceivedAppendEntries(request));
                }
            }
        }
    }

    fn send_heartbeats(&mut self, leader_id: NodeId) {
        let peers: Vec<NodeId> = {
            let leader = self.runners.get(&leader_id).unwrap();
            leader.node().peers.clone()
        };
        
        for peer_id in peers {
            let request = {
                let leader = self.runners.get(&leader_id).unwrap();
                leader.node().create_append_entries(&peer_id)
            };
            
            if let Some(target_runner) = self.runners.get_mut(&peer_id) {
                let _response = target_runner.node_mut().handle_append_entries(request);
                println!("  [HEARTBEAT] Node {:?} -> Node {:?}", leader_id, peer_id);
            }
        }
    }

    pub fn print_status(&self) {
        println!("\n=== Cluster Status ===");
        for (&id, runner) in &self.runners {
            let node = runner.node();
            println!(
                "Node {:?}: {:?} (Term: {:?})", 
                id, node.state, node.current_term
            );
        }
        println!("=====================\n");
    }
    
    pub fn find_leader(&self) -> Option<NodeId> {
        for (&id, runner) in &self.runners {
            if runner.node().is_leader() {
                return Some(id);
            }
        }
        None
    }
    
    pub fn node(&self, id: NodeId) -> Option<&RaftNode> {
        self.runners.get(&id).map(|r| r.node())
    }
}
