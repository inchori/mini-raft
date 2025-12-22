use mini_raft::simulator::Simulator;
use mini_raft::types::NodeId;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let nodes = vec![NodeId::new(1), NodeId::new(2), NodeId::new(3)];
    let mut sim = Simulator::new(nodes);
    
    println!("🚀 Starting Raft Simulation with 3 nodes");
    sim.print_status();
    
    for tick in 1..=20 {
        println!("--- Tick {} ---", tick);
        sim.tick();
        
        // 리더 선출 확인
        if let Some(leader) = sim.find_leader() {
            println!("✅ Leader elected: Node {:?}", leader);
            sim.print_status();
            break;
        }
        
        sleep(Duration::from_millis(50));
    }
    
    println!("Simulation complete!");
}