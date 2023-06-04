# axum-log-stream

An example axum server that serves a page with a live stream of the server's
tracing log.

## Setup

A `.env` file should be at the root of the directory. Leave the default or
change them to your desired values:

```bash
RUST_LOG=axum_log_stream=debug,tower_http=debug
HOST=127.0.0.1
PORT=3000
TITLE=axum-log-stream
MAX_MEM_LOG_SIZE=1000000
```

## Running

```bash
cargo run
```
