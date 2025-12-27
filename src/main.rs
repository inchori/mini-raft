use tonic::transport::Server;

use mini_raft::node::RaftNode;
use mini_raft::raft_proto::raft_server::RaftServer as RaftGrpcServer;
use mini_raft::server::RaftServer;
use mini_raft::types::NodeId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let node = RaftNode::new(NodeId::new(1), vec![]);
    let raft_server = RaftServer::new(node);

    println!("Raft server listening on {}", addr);

    Server::builder()
        .add_service(RaftGrpcServer::new(raft_server))
        .serve(addr)
        .await?;

    Ok(())
}
