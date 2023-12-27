#!/bin/bash

setcap 'cap_net_admin=eip' ./atun
./atun &
sleep 4
pid=$!

ip addr add 10.8.0.1/24 dev utun5 
ip link set up dev utun5 
ip link set dev utun5 mtu 1400

trap "kill $pid" INT TERM

wait $pid
