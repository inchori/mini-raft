use mini_raft::node::RaftNode;
use mini_raft::types::NodeId;

fn main() {
    let mut node = RaftNode::new(
        NodeId::new(1),
        vec![NodeId::new(2), NodeId::new(3)],
    );
    
    // Leader로 전환
    node.become_candidate();
    node.become_leader();
    
    println!("State: {:?}", node.state);
    
    // AppendEntries 생성
    let peer = NodeId::new(2);
    let request = node.create_append_entries(&peer);
    
    println!("\nAppendEntries to {:?}:", peer);
    println!("  term: {:?}", request.term);
    println!("  leader_id: {:?}", request.leader_id);
    println!("  prev_log_index: {:?}", request.prev_log_index);
    println!("  prev_log_term: {:?}", request.prev_log_term);
    println!("  entries: {} entries", request.entries.len());
    println!("  leader_commit: {:?}", request.leader_commit);
}
