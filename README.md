# atun
A toy VPN implemented in Rust based on https://write.yiransheng.com/vpn

# Instructions MacOS
```
make build
docker-compose up
```

This will create a docker network, `atun-test`, start the server and the client. Both containers will run the binary, which will set up a new virtual interface from which to receive and forward data.
The client will already be configured to connect to the server's public IP (172.19.0.2) of the docker network. 

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

# Instructions for remote physical machines
Let **M1** and **M2** be machine one and machine two, respectively. Let *IP1** be the public ip address of $M1$, which will be our server machine. 

Inside M1, clone the `atun` repo and run it without any argument, so that it initializes as the server

```
git clone https://github.com/lambdaclass/atun.git
cd atun
cargo run
```

Open another terminal in M1 and listen in the VPN's interface `utun5` and some port, for example 1234:

```
nc -u -l 10.8.0.1 1234
```

Inside M2, clone the repo too and run the it specifying the server's public IP address and port where it is listening. Right now the port is hardcoded to 19988:

```
git clone https://github.com/lambdaclass/atun.git
cd atun
cargo run -- --peer <IP1>:19988
```

Open another terminal in M2 and try to connect to the server with netcat via the private IP address of the VPN. Try sending the `hello?` handshake as well as any message you want

```
nc -u 10.8.0.1 1234
hello?
```

A `handshake received` log should appear in the server.


