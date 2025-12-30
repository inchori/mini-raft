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
| `--port` | Port to listen on |
| `--peers` | Comma-separated peer list: `id=host:port,...` |

## References

- [Raft Paper](https://raft.github.io/raft.pdf)
- [Raft Visualization](https://raft.github.io/)

## License

MIT
