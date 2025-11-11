use crate::{rpc::{AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse}};

#[derive(Debug, Clone)]
pub enum RaftEvent {
    ElectionTimeout,
    HeartbeatTimeout,
    ReceivedRequestVote(RequestVoteRequest),
    ReceivedRequestVoteResponse(RequestVoteResponse),
    ReceivedAppendEntries(AppendEntriesRequest),
    ReceivedAppendEntriesResponse(AppendEntriesResponse)
}
