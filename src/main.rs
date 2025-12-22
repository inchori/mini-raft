use std::thread::sleep;
use std::time::Duration;

use mini_raft::node::RaftNode;
use mini_raft::raft::RaftRunner;
use mini_raft::types::NodeId;

fn main() {
    let node = RaftNode::new(
        NodeId::new(1),
        vec![NodeId::new(2), NodeId::new(3)],
    );
    
    let mut runner = RaftRunner::new(node);
    
    println!("Initial state: {:?}", runner.node().state);
    
    sleep(Duration::from_millis(350));
    
    let actions = runner.tick();
    
    println!("State after tick: {:?}", runner.node().state);
    println!("Actions: {:?}", actions);
}
