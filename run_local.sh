#!/usr/bin/env bash
set -euo pipefail

# Build the backend binary and run it in background, then start the Python UI.
cd "$(dirname "$0")"

echo "Building Rust backend..."
cargo build --bin backend

echo "Starting backend (logs -> backend.log)..."
nohup ./target/debug/backend > backend.log 2>&1 &
BACKEND_PID=$!
sleep 0.5
echo "Backend started (PID: ${BACKEND_PID})"

echo "Launching Python UI..."
python3 ambient_assistant.py

wait
