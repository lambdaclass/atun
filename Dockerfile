# Use an official Rust runtime as a parent image
FROM rust:1.72 as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

RUN cargo build --release

FROM ubuntu
RUN apt-get install libcap2-bin
COPY --from=builder /usr/src/myapp/target/release/atun /atun
COPY scripts/run_server.sh /run_server.sh

# Run the application
CMD ["bash", "/run_server.sh"]
