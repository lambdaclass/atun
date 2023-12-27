# atun
A toy VPN implemented in Rust based on https://write.yiransheng.com/vpn

# Instructions MacOS
```
// In one terminal, run
cargo b
sudo /target/debug/atun

// In another terminal, run
sudo route -n add -net 10.0.0.0/24 10.0.0.1
ping 10.0.0.1
```

# Setting the Docker server
```
docker build --tag atun-server .
docker run -it --name atun atun-server
```
