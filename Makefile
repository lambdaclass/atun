build:
	docker build -t atun-server -f Dockerfile.server . & docker build -t atun-client -f Dockerfile.client .

run-server:
	docker run --name atun-server \
	--rm --network=atun-test --cap-add=NET_ADMIN \
	--device=/dev/net/tun atun-server:latest &

run-client:
	docker run --name atun-client \
	--rm --network=atun-test --cap-add=NET_ADMIN \
	--device=/dev/net/tun atun-client:latest &

stop:
	docker stop atun-server & docker stop atun-client
