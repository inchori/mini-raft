use std::collections::HashMap;

use crate::{
    event::RaftEvent,
    node::RaftNode,
    raft::{RaftAction, RaftRunner},
    types::{NodeId, RaftState},
};

pub struct Simulator {
    runners: HashMap<NodeId, RaftRunner>,
    vote_counts: HashMap<(NodeId, u64), usize>,
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
        let mut new_leaders: Vec<NodeId> = Vec::new();

        for (&id, runner) in &mut self.runners {
            let before_state = runner.node().state;
            let before_term = runner.node().current_term;

            let actions = runner.tick();

            let after_state = runner.node().state;
            let after_term = runner.node().current_term;

            if before_state != after_state {
                println!("  [STATE] Node {:?}: {:?} -> {:?}", id, before_state, after_state);
                if after_state != RaftState::Candidate {
                    self.vote_counts.retain(|(node_id, _), _| *node_id != id);
                }
            }

            if after_state == RaftState::Candidate && after_term != before_term {
                let term = after_term.get();
                self.vote_counts.insert((id, term), 1);
                if runner.node().quorum() == 1 {
                    runner.node_mut().become_leader();
                    println!("[LEADER] Node {:?} became Leader! (votes: 1)", id);
                    new_leaders.push(id);
                    self.vote_counts.remove(&(id, term));
                }
            }

            for action in actions {
                all_actions.push((id, action));
            }
        }

        for leader_id in new_leaders {
            self.send_heartbeats(leader_id);
        }

        for (from, action) in all_actions {
            self.handle_action(from, action);
        }
    }

    fn handle_action(&mut self, from: NodeId, action: RaftAction) {
        match action {
            RaftAction::SendRequestVote(to, request) => {
                if let Some(target_runner) = self.runners.get_mut(&to) {
                    let response = target_runner.node_mut().handle_request_vote(request);

                    if let Some(sender_runner) = self.runners.get_mut(&from) {
                        sender_runner.node_mut().handle_request_response(response.clone());
                    }

                    let sender_runner = self.runners.get(&from).unwrap();
                    let current_term = sender_runner.node().current_term.get();
                    let is_candidate = sender_runner.node().is_candidate();
                    let quorum = sender_runner.node().quorum();

                    if !is_candidate {
                        self.vote_counts.retain(|(node_id, _), _| *node_id != from);
                    }

                    if response.vote_granted && is_candidate && response.term.get() == current_term {
                        let count = self.vote_counts.entry((from, current_term)).or_insert(1);
                        *count += 1;

                        if *count >= quorum {
                            let sender_runner = self.runners.get_mut(&from).unwrap();
                            sender_runner.node_mut().become_leader();
                            println!("[LEADER] Node {:?} became Leader! (votes: {})", from, *count);

                            self.send_heartbeats(from);

                            self.vote_counts.remove(&(from, current_term));
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
                target_runner.push_event(RaftEvent::ReceivedAppendEntries(request));
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
