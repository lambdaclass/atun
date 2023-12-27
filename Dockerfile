FROM rust:1.72 as builder

COPY . .

COPY scripts/run_server.sh /run_server.sh
RUN cargo clean 

RUN cargo build --release 

FROM ubuntu:22.04

RUN apt update

RUN apt install -y iproute2 libcap2-bin netcat
COPY --from=builder /usr/src/myapp/target/release/atun /usr/local/bin/atun
COPY scripts/run_server.sh /run_server.sh

CMD ["bash", "/run_server.sh"]

