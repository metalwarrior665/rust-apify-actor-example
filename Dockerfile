# Image with prebuilt Rust. We use the newest 1.* version
# https://hub.docker.com/_/rust
FROM rust:1

COPY . .

# For production use, change to optimized build 
# RUN cargo build --release
RUN cargo build

CMD ["./target/debug/actor-example"]