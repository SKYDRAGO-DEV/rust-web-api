# rust-web-api

A compact **Rust + Axum API foundation** retained as supporting systems-engineering work.

The repository makes a deliberately narrow claim: it provides a runnable health API plus reusable JWT, error, and user-model modules. It is **not** presented as a production-ready application, benchmarked high-throughput service, complete authentication platform, or deployed financial system.

## Implemented

- Axum HTTP server
- `GET /api/v1/health`
- Structured tracing
- Configurable IP/port binding through `HOST` and `PORT`
- Graceful shutdown on Ctrl+C and SIGTERM
- JWT token creation and verification helpers
- JWT round-trip and invalid-secret tests
- Reusable user/domain models
- Structured application error type
- CI gates for rustfmt, Clippy, compilation, and tests

## Run

Requirements:

- Rust 1.80+

```bash
cargo run
```

Defaults:

```text
HOST=0.0.0.0
PORT=8080
```

Override them explicitly when required:

```bash
HOST=127.0.0.1 PORT=9000 cargo run
```

`HOST` is intentionally parsed as an IP address rather than an arbitrary hostname, and `PORT` must be between 1 and 65535. Invalid bind configuration fails before the server starts.

Health endpoint with the defaults:

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
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets
cargo test --all-targets
```

CI executes these quality gates for pushes and pull requests.

## Architecture

```text
src/
├── main.rs          # Axum service, bind validation, health handler, shutdown handling
├── lib.rs           # reusable library exports
├── auth/
│   ├── mod.rs
│   └── jwt.rs       # JWT encode/verify helpers + tests
├── models/
│   ├── mod.rs
│   └── user.rs      # user DTO/domain models
└── error.rs         # API error representation
```

## Operational behavior

The executable validates bind configuration before creating the listener. Once running, it listens for Ctrl+C and, on Unix, SIGTERM; either signal triggers Axum's graceful-shutdown path so the server can stop accepting new work while in-flight requests finish according to the framework's behavior.

This improves deployability, but it is not a complete production lifecycle system: there are no readiness-drain hooks, external load-balancer coordination, database shutdown hooks, or deployment manifests in this repository.

## Scope boundaries

Earlier documentation claimed features that the repository did not contain, including:

- 100k+ requests/second benchmark results
- complete login/register/user/item endpoints
- OpenAPI generation
- rate limiting
- SQLx query guarantees and complete migration infrastructure
- a passed RustSec security audit
- production readiness

Those claims remain intentionally excluded.

The current executable does **not** require a database or migrations. The SQLx-backed user model and database-aware error variant are retained as reusable code but are not wired into the health service.

## Security notes

- JWT secrets are function inputs; no production secret is hardcoded into the application.
- Test secrets are non-production fixtures only.
- Future authentication or database integration should load credentials through an external secret mechanism or environment variables.
- Authentication, authorization, CORS policy, rate limiting, TLS termination, database migrations, dependency-vulnerability policy, and deployment hardening would all require additional work before real production use.

## Portfolio role

This repository demonstrates Rust/Axum foundations, typed backend engineering, CI discipline, and basic service lifecycle handling. It is supporting work, not a flagship Forex/Quant repository.
