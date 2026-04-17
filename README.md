# rust-web-api

> High-performance REST API boilerplate built with Rust + Axum | Async, typed, production-ready

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust)](https://rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-4A5568?style=flat-square)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=flat-square&logo=postgresql)](https://www.postgresql.org/)
[![Rustsec](https://img.shields.io/badge/Security-Audit-passed-brightgreen)](https://rustsec.org/)

## Features

- **Blazing Fast** — Built on tokio async runtime, handles 100k+ req/s
- **Type-Safe** — Full compile-time guarantees with Rust's type system
- **Migration-Safe** — SQLx compile-time checked queries, zero runtime SQL errors
- **JWT Auth** — Secure stateless authentication out of the box
- **OpenAPI** — Auto-generated API docs via utoipa + utoipa-gen
- **Rate Limiting** — Built-in request throttling for API protection

## Tech Stack

- **Runtime:** Tokio
- **Framework:** Axum
- **ORM:** SQLx (compile-time checked)
- **Auth:** JWT (jsonwebtoken)
- **Validation:** validator + refined
- **Serialization:** serde + chrono

## Quick Start

\`\`\`bash
# Development
cargo run

# Production
cargo build --release
RUST_LOG=info ./target/release/rust-web-api

# Run tests
cargo test -- --nocapture
\`\`\`

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/health` | Health check |
| POST | `/api/v1/auth/login` | User login |
| POST | `/api/v1/auth/register` | User registration |
| GET | `/api/v1/users/me` | Get current user |
| PUT | `/api/v1/users/me` | Update user |
| GET | `/api/v1/items` | List items (paginated) |
| POST | `/api/v1/items` | Create item |
| GET | `/api/v1/items/:id` | Get item |
| DELETE | `/api/v1/items/:id` | Delete item |

## Architecture

\`\`\`
src/
├── api/           # Route handlers & middleware
├── db/            # Database layer (SQLx)
├── models/        # Domain models & DTOs
├── services/      # Business logic
├── auth/          # JWT authentication
└── error/         # Error types & handling
\`\`\`

## License

MIT © SKYDRAGO-DEV
