# Ambient Assistant — Local run instructions

This workspace uses a Rust backend and a Python UI. The backend exposes a small HTTP API at `http://localhost:8080/suggestions` (implemented in `backend.rs`). The UI is `ambient_assistant.py`.

Quick start (local development):

1. Build and run both backend and UI together using the helper script:

```bash
./run_local.sh
```

2. Or run components separately:

Start backend:

```bash
cargo run --bin backend
# or build then run the binary in background
cargo build --bin backend
./target/debug/backend &
```

Start UI:

```bash
python3 ambient_assistant.py
```

Notes:
- The UI expects the backend at `http://localhost:8080/suggestions`. If you see connection refused errors, ensure the backend is running.
- Logs from the helper script go to `backend.log`.
# ambient-assistant
