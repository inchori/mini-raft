#!/bin/bash

# E2E Test: Run 3-node Raft cluster

set -e

BINARY="./target/release/mini-raft"
LOG_DIR="./logs"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

cleanup() {
    echo -e "${YELLOW}Stopping all nodes...${NC}"
    pkill -f "mini-raft --id" 2>/dev/null || true
    sleep 1
}

trap cleanup EXIT

# Build
echo -e "${YELLOW}Building release...${NC}"
cargo build --release

# Create log directory
mkdir -p "$LOG_DIR"
rm -f "$LOG_DIR"/*.log

# Start nodes with delay
echo -e "${YELLOW}Starting Node 1...${NC}"
$BINARY --id 1 --port 50051 --peers "2=[::1]:50052,3=[::1]:50053" > "$LOG_DIR/node1.log" 2>&1 &
NODE1_PID=$!
sleep 2

echo -e "${YELLOW}Starting Node 2...${NC}"
$BINARY --id 2 --port 50052 --peers "1=[::1]:50051,3=[::1]:50053" > "$LOG_DIR/node2.log" 2>&1 &
NODE2_PID=$!
sleep 2

echo -e "${YELLOW}Starting Node 3...${NC}"
$BINARY --id 3 --port 50053 --peers "1=[::1]:50051,2=[::1]:50052" > "$LOG_DIR/node3.log" 2>&1 &
NODE3_PID=$!

echo -e "${GREEN}All nodes started!${NC}"
echo "  Node 1 PID: $NODE1_PID"
echo "  Node 2 PID: $NODE2_PID"
echo "  Node 3 PID: $NODE3_PID"
echo ""
echo "Logs: $LOG_DIR/"
echo ""

# Wait for leader election
echo -e "${YELLOW}Waiting for leader election (5 seconds)...${NC}"
sleep 5

# Check for leaders
echo -e "${YELLOW}Checking for leaders...${NC}"
LEADER_COUNT=$(grep -l "Became Leader" "$LOG_DIR"/*.log 2>/dev/null | wc -l)
echo "Leader count: $LEADER_COUNT"

if [ "$LEADER_COUNT" -eq 1 ]; then
    LEADER_NODE=$(grep -l "Became Leader" "$LOG_DIR"/*.log | head -1)
    echo -e "${GREEN}SUCCESS: Exactly 1 leader elected!${NC}"
    echo "Leader: $LEADER_NODE"
else
    echo -e "${RED}FAIL: Expected 1 leader, found $LEADER_COUNT${NC}"
    echo "Check logs for details:"
    for f in "$LOG_DIR"/*.log; do
        echo "=== $f ==="
        grep -E "(Started election|Became Leader|Received vote)" "$f" | tail -10
    done
fi

echo ""
echo -e "${YELLOW}Press Ctrl+C to stop the cluster${NC}"

# Keep running
wait

