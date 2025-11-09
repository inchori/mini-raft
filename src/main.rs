use mini_raft::{log::{LogEntry, LogStore}, node::RaftNode, types::{LogIndex, NodeId, Term}};

fn main() {
    let node = RaftNode::new(
        NodeId::new(1),
        vec![NodeId::new(2), NodeId::new(3), NodeId::new(4)],
    );
    
    println!("Node ID: {:?}", node.id);
    println!("State: {:?}", node.state);
    println!("Cluster size: {}", node.cluster_size());
    println!("Quorum: {}", node.quorum());
    println!("Is follower? {}", node.is_follower());
}
