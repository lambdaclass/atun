#!/bin/bash

sudo setcap cap_net_admin=eip target/debug/atun

target/release/atun --peer 172.18.0.2:19988 &

pid=$!

# FIXME: Check how to do in MacOS
# sudo ip addr add 10.8.0.3/24 dev tun0
# sudo ip link set up dev tun0
# sudo ip link set dev tun0 mtu 1400

trap "kill $pid" INT TERM

wait $pid