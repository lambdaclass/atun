FROM rust:1.72 as builder

COPY . .

COPY scripts/run_server.sh /run_server.sh
RUN cargo clean 

RUN cargo build --release 

FROM ubuntu
RUN apt-get install libcap2-bin
COPY --from=builder /usr/src/myapp/target/release/atun /atun
COPY scripts/run_server.sh /run_server.sh

CMD ["bash", "/run_server.sh"]

