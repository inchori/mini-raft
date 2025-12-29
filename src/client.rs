use std::collections::HashMap;

use tonic::Response;
use tonic::transport::Channel;

use crate::raft_proto::{AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse};
use crate::types::NodeId;

use crate::raft_proto::raft_client::RaftClient as RaftGrpcClient;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct RaftClient {
    peers: HashMap<NodeId, String>,
    //TODO: use connections cache later
    // connections: HashMap<NodeId, RaftGrpcClient<Channel>>,
}

impl RaftClient {
    pub fn new(peers: HashMap<NodeId, String>) -> Self {
        Self {
            peers,
        }
    }

    pub async fn send_request_vote(&mut self, to: NodeId, req: RequestVoteRequest) -> Result<Response<RequestVoteResponse>, BoxError> {
        let addr = self.peers.get(&to).ok_or("Peer not found")?;
        let channel = Channel::from_shared(addr.clone())?.connect().await?;
        let mut client = RaftGrpcClient::new(channel);

        let resp = client.request_vote(req).await?;

        Ok(resp)
    }

    pub async fn send_append_entries(&mut self, to: NodeId, req: AppendEntriesRequest) -> Result<Response<AppendEntriesResponse>, BoxError> {
        let addr = self.peers.get(&to).ok_or("Peer not found")?;
        let channel = Channel::from_shared(addr.clone())?.connect().await?;
        let mut client = RaftGrpcClient::new(channel);

        let resp = client.append_entries(req).await?;

        Ok(resp)
    }
}