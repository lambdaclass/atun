#!/bin/bash

setcap 'cap_net_admin=eip' ./atun

./atun &

pid=$!

ip addr add 10.8.0.1/24 dev atun0
ip link set up dev atun0
ip link set dev atun0 mtu 1400

trap "kill $pid" INT TERM

wait $pid
