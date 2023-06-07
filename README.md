# axum-log-stream

An example axum server that serves a live stream of its own tracing log to an HTML page using [Hotwire Turbo Streams](https://turbo.hotwired.dev/handbook/streams).

https://github.com/thallada/axum-log-stream/assets/1505923/505b4d85-748c-4bb7-992d-d6f1dac314ad

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
