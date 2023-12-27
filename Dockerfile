# Use an official Rust runtime as a parent image
FROM rust:1.72 as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

RUN cargo build --release

FROM ubuntu
RUN apt-get update && apt-get install -y extra-runtime-dependencies
COPY --from=builder /usr/src/myapp/target/release/atun /usr/local/bin/atun
COPY scripts/run_server.sh /run_server.sh

# Run the application
CMD ["bash", "/run_server.sh"]
