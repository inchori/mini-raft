# mini-raft

A minimal implementation of the Raft consensus algorithm in Rust for learning purposes.

## Features

- [x] Leader Election
- [x] Heartbeat mechanism
- [x] Request Vote RPC
- [x] Append Entries RPC
- [x] Log replication
- [x] gRPC network layer
- [x] CLI & Config
- [x] E2E Test script
- [x] HTTP Client API
- [ ] Log Snapshot (Optional)
- [ ] Persistence (Optional)

## Project Structure

```
src/
├── types.rs      # Core types (Term, NodeId, LogIndex, RaftState)
├── rpc.rs        # RPC messages (RequestVote, AppendEntries)
├── log.rs        # Log entry and storage
├── timer.rs      # Election and heartbeat timers
├── event.rs      # Event types for the event loop
├── node.rs       # RaftNode - core Raft logic
├── raft.rs       # RaftRunner - event loop with network calls
├── server.rs     # gRPC server (receives RPCs)
├── client.rs     # gRPC client (sends RPCs)
├── http.rs       # HTTP API server
├── lib.rs        # Module exports
└── main.rs       # CLI entry point

proto/
└── raft.proto    # gRPC service definition
```

## Quick Start

Build the project:

```bash
cargo build --release
```

Run a 3-node cluster (in separate terminals):

```bash
# Terminal 1 - Node 1
./target/release/mini-raft --id 1 --port 50051 --peers "2=[::1]:50052,3=[::1]:50053"

# Terminal 2 - Node 2
./target/release/mini-raft --id 2 --port 50052 --peers "1=[::1]:50051,3=[::1]:50053"

# Terminal 3 - Node 3
./target/release/mini-raft --id 3 --port 50053 --peers "1=[::1]:50051,2=[::1]:50052"
```

## CLI Options

| Option | Description |
|--------|-------------|
| `--id` | Unique node ID |
| `--port` | Port to listen on (gRPC) |
| `--peers` | Comma-separated peer list: `id=host:port,...` |

## HTTP API

Each node exposes an HTTP API on port `gRPC_port + 1000` (e.g., 50051 -> 51051).

### GET /status

Returns the current node status.

```bash
curl http://[::1]:51051/status
```

Response:
```json
{
  "node_id": 1,
  "state": "Leader",
  "term": 5,
  "commit_index": 3
}
```

### POST /command

Submit a command to the cluster (leader only).

```bash
curl -X POST http://[::1]:51051/command \
  -H "Content-Type: application/json" \
  -d '{"command": "set key value"}'
```

Response (success):
```json
{
  "success": true,
  "index": 4,
  "error": null
}
```

Response (not leader):
```json
{
  "success": false,
  "index": null,
  "error": "not_leader"
}
```

> **TODO**: Future versions will include `leader_hint` field to redirect clients to the current leader.

### GET /log

Returns all log entries.

```bash
curl http://[::1]:51051/log
```

Response:
```json
[
  {"index": 1, "term": 1, "command": "set foo bar"},
  {"index": 2, "term": 1, "command": "set baz qux"}
]
```

## References

- [Raft Paper](https://raft.github.io/raft.pdf)
- [Raft Visualization](https://raft.github.io/)

## License

MIT
