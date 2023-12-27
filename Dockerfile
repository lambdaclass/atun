FROM ubuntu:22.04

RUN apt update

RUN apt install -y iproute2 libcap2-bin netcat

COPY target/debug/atun /atun 

COPY scripts/run_server.sh /run_server.sh

CMD bash run_server.sh
