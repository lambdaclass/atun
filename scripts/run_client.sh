#!/bin/bash

# FIXME: Instead of setting these capabilities, we are going to run the script with sudo. 
# sudo setcap cap_net_admin=eip target/debug/atun

target/debug/atun --peer 172.18.0.2:19988 &

pid=$!

# FIXME: Check how to do in MacOS
sudo ip addr add 10.8.0.3/24 dev utun4
sudo ip link set up dev utun4
sudo ip link set dev utun4 mtu 1400

trap "kill $pid" INT TERM

wait $pid