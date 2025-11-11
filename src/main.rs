use mini_raft::node::RaftNode;
use mini_raft::types::NodeId;

fn main() {
    let mut node = RaftNode::new(
        NodeId::new(1),
        vec![NodeId::new(2), NodeId::new(3)],
    );
    
    node.become_candidate();
    println!("Candidate: {:?}", node.state);
    
    node.become_leader();
    println!("Leader: {:?}", node.state);
    println!("next_index: {:?}", node.next_index);
}