# rust-streaming-example
An example of streaming data in Rust.

## Features
- [x] Server-Sent Events (SSE)

## Run
```bash
cargo run
```

## Test
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Accept: text/event-stream" \
  -d '{
    "model": "test-model",
    "messages": [
      {"role": "user", "content": "Hello, this is a streaming test message"}
    ],
    "stream": true
  }'
```

## Reference
- [Rust SSE Example](https://github.com/poem-web/poem/tree/main/examples/sse)
- [Rust SSE Example](https://github.com/poem-web/poem/tree/main/examples/sse)
