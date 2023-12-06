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
# test
TEST_LOG=true cargo test | bunyan

# dev
RUST_LOG=trace cargo watch -x check -x "run | bunyan"
```
