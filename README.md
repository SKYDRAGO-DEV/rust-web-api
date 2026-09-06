# rust-web-api

A compact **Rust + Axum API foundation** retained as supporting systems-engineering work.

The repository now makes a deliberately narrow claim: it provides a runnable health API plus reusable JWT, error, and user-model modules. It is **not** presented as a production-ready application, benchmarked high-throughput service, complete authentication platform, or deployed financial system.

## Implemented

- Axum HTTP server
- `GET /api/v1/health`
- Structured tracing
- JWT token creation and verification helpers
- JWT round-trip and invalid-secret tests
- Reusable user/domain models
- Structured application error type
- Strict CI for formatting, compilation, and tests

## Run

Requirements:

- Rust 1.80+

```bash
cargo run
```

The server binds to:

```text
0.0.0.0:8080
```

Health endpoint:

```bash
curl http://127.0.0.1:8080/api/v1/health
```

Example response shape:

```json
{
  "status": "healthy",
  "service": "rust-web-api",
  "version": "0.1.0",
  "timestamp": "..."
}
```

## Test and validate

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

CI executes the same quality gates.

## Architecture

```text
src/
├── main.rs          # runnable Axum service + health handler
├── lib.rs           # reusable library exports
├── auth/
│   ├── mod.rs
│   └── jwt.rs       # JWT encode/verify helpers + tests
├── models/
│   ├── mod.rs
│   └── user.rs      # user DTO/domain models
└── error.rs         # API error representation
```

## Scope boundaries

The previous version of this README claimed features that the repository did not contain, including:

- 100k+ requests/second benchmark results
- complete login/register/user/item endpoints
- OpenAPI generation
- rate limiting
- SQLx query guarantees and complete migration infrastructure
- a passed RustSec security audit
- production readiness

Those claims have been removed.

The current executable intentionally does **not** require a database or migrations. The SQLx-backed user model and database-aware error variant are retained as reusable code but are not wired into the health service.

## Security notes

- JWT secrets are function inputs; no secret is hardcoded into the application.
- Test secrets are non-production fixtures only.
- Future authentication or database integration should load credentials through an external secret mechanism or environment variables.
- Authentication, authorization, CORS policy, rate limiting, TLS termination, database migrations, and deployment hardening would all require additional work before real production use.

## Portfolio role

This repository demonstrates Rust/Axum foundations and typed backend engineering. It is supporting work, not a flagship Forex/Quant repository.
