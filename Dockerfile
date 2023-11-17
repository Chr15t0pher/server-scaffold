# Intermediate stage, builder stage does not contribute to the final image size, discarded at the end of the build.
ARG RUST_VERSION=1.72.0
ARG APP_NAME=server-scaffold
FROM rust:${RUST_VERSION} as builder
WORKDIR /app
COPY Cargo.* .
RUN cargo fetch
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server-scaffold server-scaffold

COPY config config
EXPOSE 8081
ENV APP_ENVIRONMENT=production
ENTRYPOINT [ "./server-scaffold" ]
