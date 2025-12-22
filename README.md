# mini-raft

A minimal implementation of the Raft consensus algorithm in Rust for learning purposes.

## Features

- [x] Leader Election
- [x] Heartbeat mechanism
- [x] Request Vote RPC
- [x] Append Entries RPC
- [x] Multi-node simulation
- [ ] Log replication (partial)
- [ ] Persistence

## Project Structure

```
src/
├── types.rs      # Core types (Term, NodeId, LogIndex, RaftState)
├── rpc.rs        # RPC messages (RequestVote, AppendEntries)
├── log.rs        # Log entry and storage
├── timer.rs      # Election and heartbeat timers
├── event.rs      # Event types for the event loop
├── node.rs       # RaftNode - core Raft logic
├── raft.rs       # RaftRunner - event loop wrapper
├── simulator.rs  # Multi-node cluster simulation
├── lib.rs        # Module exports
└── main.rs       # Example simulation
```

## Quick Start

```bash
cargo run
```

## References

- [Raft Paper](https://raft.github.io/raft.pdf)
- [Raft Visualization](https://raft.github.io/)

## License

MIT
