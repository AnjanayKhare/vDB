# vDB - Vector Database

A lightweight vector database server built with Rust and Axum, designed for efficient storage and retrieval of vector embeddings.

## Overview

vDB is a RESTful API server for managing vector data and collections. It provides endpoints to create collections and store/retrieve vector embeddings efficiently.

## Features

- **REST API** - Easy-to-use HTTP endpoints for vector operations
- **Collections** - Organize vectors into named collections
- **Async Runtime** - Built on Tokio for high-performance async operations
- **Type-Safe** - Leverages Rust's type system for safety and reliability
- **JSON Support** - Seamless JSON serialization/deserialization

## Tech Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8.9
- **Async Runtime**: Tokio 1.52.1 (with full features)
- **Serialization**: Serde 1.0.228 + serde_json 1.0.149
- **Date/Time**: Chrono 0.4.44

## Prerequisites

- Rust 1.75+ (with Cargo)
- Windows, macOS, or Linux

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd vDB
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## API Endpoints

### Health Check
```
GET /
```
Response:
```json
{
  "status": "ok",
  "message": "Hello, World!"
}
```

### Add Vector to Collection
```
GET /collections
```
Request Body:
```json
{
  "data": [1.0, 2.0, 3.0],
  "collection": "my_collection"
}
```

Response:
```json
{
  "data": [1.0, 2.0, 3.0],
  "collection": "my_collection"
}
```

## Project Structure

```
vDB/
├── src/
│   ├── main.rs              # Application entry point
│   ├── models/
│   │   ├── mod.rs           # Models module
│   │   ├── vector_manager.rs # Vector and collection request types
│   │   └── utils.rs         # Utility functions (Logger)
│   └── routes/
│       ├── mod.rs           # Routes module
│       └── collections.rs   # Collection-related routes
├── Cargo.toml               # Project dependencies
└── README.md                # This file
```

## Development

### Running in Development Mode
```bash
cargo run
```

### Running with Logging
The application includes a custom Logger utility for debugging information.

### Building Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

## Configuration

The server binds to `127.0.0.1:3000` by default. To change this, modify the `PORT` constant in `src/main.rs`.

## Future Enhancements

- [ ] Persistent storage (database backend)
- [ ] Vector similarity search (cosine similarity, etc.)
- [ ] Authentication and authorization
- [ ] Collection management endpoints (create, delete, list)
- [ ] Pagination support
- [ ] Performance optimization
- [ ] Comprehensive error handling
- [ ] Integration tests

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

[Add your license here]

## Author

Created as a learning project for vector database concepts in Rust.
