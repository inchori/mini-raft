use mini_raft::node::RaftNode;
use mini_raft::rpc::RequestVoteRequest;
use mini_raft::types::{NodeId, Term, LogIndex};

fn main() {
    let mut node = RaftNode::new(
        NodeId::new(1),
        vec![NodeId::new(2), NodeId::new(3)],
    );
    
    println!("Initial state: {:?}", node.state);
    println!("Initial term: {:?}", node.current_term);
    println!("Voted for: {:?}", node.voted_for);
    
    // 투표 요청 테스트
    let request = RequestVoteRequest {
        term: Term::new(1),
        candidate_id: NodeId::new(2),
        last_log_index: LogIndex::ZERO,
        last_log_term: Term::ZERO,
    };
    
    let response = node.handle_request_vote(request);
    println!("\nAfter vote request:");
    println!("Vote granted: {}", response.vote_granted);
    println!("Voted for: {:?}", node.voted_for);
}