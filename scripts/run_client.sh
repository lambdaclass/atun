#!/bin/bash

setcap cap_net_admin=eip ./atun
./atun --peer 172.19.0.2:19988 &

pid=$!

# FIXME: Check how to do in MacOS
ip addr add 10.8.0.3/24 dev utun5 
ip link set up dev utun5 
ip link set dev utun5 mtu 1400

trap "kill $pid" INT TERM

wait $pid
