use std::sync::{Arc, Mutex};

use axum::{Json, Router, extract::State, routing::{get, post}};
use serde::{Deserialize, Serialize};

use crate::node::RaftNode;

pub struct AppState {
    pub node: Arc<Mutex<RaftNode>>,
}

pub async fn get_status(State(state): State<Arc<AppState>>) -> Json<StatusResponse> {
    let node = state.node.lock().unwrap();

    Json(StatusResponse {
        node_id: node.id.get(),
        state: format!("{:?}", node.state),
        term: node.current_term.get(),
        commit_index: node.commit_index.get(),
    })
}

pub async fn command(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CommandRequest>,
) -> Json<CommandResponse> {
    let mut node = state.node.lock().unwrap();

    match node.client_request(req.command.into_bytes()) {
        Some(index) => Json(CommandResponse {
            success: true,
            index: Some(index.get()),
            error: None,
        }),
        None => Json(CommandResponse {
            success: false,
            index: None,
            error: Some("not_leader".to_string()),
        }),
    }
}

pub async fn get_log(State(state): State<Arc<AppState>>) -> Json<Vec<LogEntryResponse>> {
    let node = state.node.lock().unwrap();

    let entries: Vec<LogEntryResponse> = node
        .log
        .entries
        .iter()
        .map(|e| LogEntryResponse {
            index: e.index.get(),
            term: e.term.get(),
            command: String::from_utf8_lossy(&e.command).to_string(),
        })
        .collect();

    Json(entries)
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/status", get(get_status))
        .route("/command", post(command))
        .route("/log", get(get_log))
        .with_state(state)
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub node_id: u64,
    pub state: String,
    pub term: u64,
    pub commit_index: u64,
}

#[derive(Deserialize)]
pub struct CommandRequest {
    pub command: String,
}

#[derive(Serialize)]
pub struct CommandResponse {
    pub success: bool,
    pub index: Option<u64>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct LogEntryResponse {
    pub index: u64,
    pub term: u64,
    pub command: String,
}
