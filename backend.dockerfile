FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin evil-backend

# # We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/evil-backend /usr/local/bin
RUN apt-get update && apt install -y openssl libpq5 ca-certificates
ENTRYPOINT ["/usr/local/bin/evil-backend"]

# 2. For Nginx setup
# FROM nginx:alpine

# # Copy config nginx
# COPY --from=builder /app/.nginx/nginx.conf /etc/nginx/conf.d/default.conf

# WORKDIR /usr/share/nginx/html

# # Remove default nginx static assets
# RUN rm -rf ./*

# COPY --from=builder /app/target/release/evil-backend /usr/local/bin
# ENTRYPOINT ["nginx", "-g", "daemon off;"]