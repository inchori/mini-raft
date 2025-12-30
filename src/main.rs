use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}, time::Duration};
use clap::Parser;
use mini_raft::{node::RaftNode, raft::RaftRunner, raft_proto::raft_server::RaftServer as RaftGrpcServer, server::RaftServer, types::NodeId};
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "miniraft")]
#[command(about = "A minimal Raft consensus implementation")]
struct Args {
    #[arg(long)]
    id: u64,

    #[arg(long)]
    port: u16,

    #[arg(long)]
    peers: String,
}

fn parse_peers(peer_str: &str) -> HashMap<NodeId, String> {
    let mut peers= HashMap::new();

    for p in peer_str.split(',') {
        let parts: Vec<&str> = p.split('=').collect();
        let id: u64 = parts[0].parse().unwrap();
        let addr = format!("http://{}", parts[1]);
        peers.insert(NodeId::new(id), addr);
    }

    peers
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("mini_raft=debug".parse()?))
        .with_target(false)
        .init();

    let args = Args::parse();

    let node_id = NodeId::new(args.id);
    let peer_addrs = parse_peers(&args.peers);
    let peers: Vec<NodeId> = peer_addrs.keys().copied().collect();
    let addr: SocketAddr = format!("[::1]:{}", args.port).parse()?;

    info!(node_id = args.id, port = args.port, "Starting Raft node");

    let node = Arc::new(Mutex::new(RaftNode::new(node_id, peers)));
    let mut runner = RaftRunner::new(Arc::clone(&node), peer_addrs.clone());

    let raft_server = RaftServer::new_with_shared(Arc::clone(&node));

    let _server_handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RaftGrpcServer::new(raft_server))
            .serve(addr)
            .await
    });

    loop {
        runner.tick().await;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
}
