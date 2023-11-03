# Server Scaffold

Server scaffold built leveraging the Actix ecosystem.

## Prerequisites

Make sure that `sqlx-cli` and `PostgreSQL` are properly installed.

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres

brew install postgresql
```

## Development

```bash
RUST_LOG=info cargo watch -x check -x "test | bunyan" -x "run | bunyan"
```
