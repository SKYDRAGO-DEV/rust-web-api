# Contributing to rust-web-api

This is a compact Rust/Axum reference application. Contributions should preserve the narrow, testable scope stated in [README.md](README.md).

## Local verification

Install a compatible Rust toolchain (README currently specifies Rust 1.80+). From the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets
cargo test --all-targets
```

The CI pipeline runs these checks. Please fix errors instead of disabling checks.

## Pull requests

- Document the defect or use case and add tests when behaviour changes.
- Preserve typed configuration validation and graceful shutdown.
- Validate JWT edge cases and do not include real signing secrets or tokens.
- Document new runtime permissions, network exposure, dependency and compatibility changes.
- Do not describe unimplemented database, authentication, performance or deployment features as shipped.
- Do not claim throughput or security audit results without reproducible evidence.

Please follow [SECURITY.md](SECURITY.md) for sensitive disclosures.
