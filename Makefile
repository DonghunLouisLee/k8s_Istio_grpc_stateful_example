	
docker-build-server:
	cd poc-server && docker build -t poc-server .

docker-build-client:
	cd poc-client && docker build -t poc-client .