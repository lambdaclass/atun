#!/bin/bash

setcap 'cap_net_admin=eip' ./atun

./atun &

pid=$!

ip addr add 10.8.0.1/24 dev utun4
ip link set up dev utun4
ip link set dev utun4 mtu 1400

trap "kill $pid" INT TERM

wait $pid
