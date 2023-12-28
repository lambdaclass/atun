# atun
A toy VPN implemented in Rust based on https://write.yiransheng.com/vpn

# Instructions MacOS
```
make build
docker-compose up
```

This will start the server and the client. Both containers will run the binary, which will set up a new interface from which to receive and forward data.
The client will already be configured to connect to the server's "public" IP (172.19.0.2). 
To connect to the VPN server: 

```
docker exec -it atun-server /bin/bash
```

To connect to the VPN client: 

```
docker exec -it atun-client /bin/bash
```

Because the VPN will be set up through the new interfaces, you can start listening on the server's private IP (UDP only) and receive data from the client:

In the server:
```
nc -u -l 10.8.0.1 4444
```

In the client:

```
nc -u 10.8.0.1 4444
```

Anything sent through the new private IP will be received on the server (and forwarded). In order for the server to recognize the client as a client of the VPN, the protocol is currently to send "hello?" which will complete the handshake.
