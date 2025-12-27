use std::sync::{Arc, Mutex};

use tonic::{Request, Response, Status};

use crate::node::RaftNode;
use crate::raft_proto::{
    raft_server::Raft,
    AppendEntriesRequest, AppendEntriesResponse,
    RequestVoteRequest, RequestVoteResponse,
};
use crate::types::{LogIndex, NodeId, Term};

pub struct RaftServer {
    node: Arc<Mutex<RaftNode>>,
}

impl RaftServer {
    pub fn new(node: RaftNode) -> Self {
        Self {
            node: Arc::new(Mutex::new(node)),
        }
    }
}

#[tonic::async_trait]
impl Raft for RaftServer {
    async fn request_vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteResponse>, Status> {
        let req = request.into_inner();

        let internal_req = crate::rpc::RequestVoteRequest {
            term: Term::new(req.term),
            candidate_id: NodeId::new(req.candidate_id),
            last_log_index: LogIndex::new(req.last_log_index),
            last_log_term: Term::new(req.last_log_term)
        };

        let internal_resp = {
            let mut node = self.node.lock().unwrap();
            node.handle_request_vote(internal_req)
        };

        let resp = RequestVoteResponse {
            term: internal_resp.term.get(),
            vote_granted: internal_resp.vote_granted,
        };

        Ok(Response::new(resp))
    }

    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesResponse>, Status> {
        let req = request.into_inner();

        let entries: Vec<crate::log::LogEntry> = req.entries
            .into_iter()
            .map(|e| crate::log::LogEntry {
                term: Term::new(e.term),
                index: LogIndex::new(e.index),
                command: e.command,
            })
            .collect();

        let internal_req = crate::rpc::AppendEntriesRequest {
            term: Term::new(req.term),
            leader_id: NodeId::new(req.leader_id),
            prev_log_index: LogIndex::new(req.prev_log_index),
            prev_log_term: Term::new(req.prev_log_term),
            entries,
            leader_commit: LogIndex::new(req.leader_commit),
        };

        let internal_resp = {
            let mut node = self.node.lock().unwrap();
            node.handle_append_entries(internal_req)
        };

        let resp = AppendEntriesResponse {
            term: internal_resp.term.get(),
            success: internal_resp.success,
        };

        Ok(Response::new(resp))
    }
}
