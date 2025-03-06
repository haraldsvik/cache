# Pseudo Cache Service

A high-performance caching service implemented in Rust that provides both HTTP and gRPC interfaces. This service is designed to handle key-value lookups efficiently using an in-memory cache.

Designed as a "Read-Only" cache that.

## Features

- HTTP API with JSON endpoints
- gRPC service interface
- In-memory cache using DashMap for concurrent access
- CORS support for web clients
- Mock data generation utility
- Efficient concurrent lookups

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Project Structure

```
.
├── src/
│   ├── main.rs         # Main HTTP server implementation
│   ├── client.rs       # gRPC client implementation
│   ├── generate_mock.rs # Mock data generator
│   └── grpc/          # gRPC service implementation
├── proto/             # Protocol buffer definitions
├── mock_data.txt      # Sample data file
└── Cargo.toml         # Project dependencies and configuration
```

## Building the Project

```bash
cargo build --release
```

## Running the Service

1. First, generate mock data (optional):
```bash
cargo run --release --bin generate-mock
```

2. Start the main service:
```bash
cargo run --release --bin cache-pseudo
```

The service will start on `http://localhost:3000` by default.

## API Endpoints

### HTTP API

#### POST /lookup
Look up multiple keys in the cache.

Request body:
```json
{
    "keys": ["1", "2", "3"]
}
```

Response:
```json
{
    "found": {
        "1": "value1",
        "2": "value2"
    },
    "missing": ["3"]
}
```

### gRPC Service

The service also exposes a gRPC interface for more efficient communication. The gRPC service runs on the same port as the HTTP service.

Use the gRPC-client to test the grpc endpoint:
```sh
cargo run --release --bin grpc-client
```

## Development

To run tests:
```bash
cargo test
```

## Dependencies

- `axum`: Web framework for HTTP API
- `tokio`: Async runtime
- `dashmap`: Concurrent hash map implementation
- `tonic`: gRPC framework
- `serde`: Serialization/deserialization
- `tower-http`: HTTP middleware (CORS support)

## License

BEERWARE

## Contributing

[Your contribution guidelines] 