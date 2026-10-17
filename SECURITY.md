# Security policy

## Current status

This is a **Rust/Axum API foundation and testable reference**, not a production authentication or multi-tenant platform. Its runnable endpoint is a health service. There is no commitment to provide security fixes on a production support schedule.

## Report vulnerabilities privately

Do not open a public issue with exploit instructions, JWT signing secrets, authentication tokens, private data or sensitive logs.

If the repository Security page offers **Report a vulnerability**, submit the report there. If that private channel is not enabled, open only a metadata-level issue asking the maintainer to establish a confidential reporting channel, without vulnerability details. Vulnerability-reporting configuration must be checked separately in GitHub settings.

## Areas requiring caution

- JWT utilities do not constitute complete authentication or authorization.
- HTTP exposure is not equivalent to production hardening; configure TLS, trusted ingress, rate limiting and authorization before any real deployment.
- Binding to `0.0.0.0` exposes the service on accessible interfaces; for local evaluation use `HOST=127.0.0.1`.
- Do not commit signing keys, `.env` secrets, database credentials or real tokens.
- A passing CI job is **not** proof that every security issue has been eliminated.

A helpful private report includes affected version/commit, environment, impact and non-sensitive reproduction information.
