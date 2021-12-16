# Image with prebuilt Rust. We use the newest 1.* version
# https://hub.docker.com/_/rust
FROM rust:1

# We copy only package setup so we cache building all dependencies
COPY Cargo* ./

# We need to have dummy main.rs file to be able to build
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release

# Delete dummy main.rs
RUN rm -rf src

# Copy rest of the files
COPY . ./

# Build the source files
RUN cargo build --release

CMD ["./target/release/actor-example"]