	
docker-build-server:
	cd server && docker build -t poc-server .

docker-build-client:
	cd client && docker build -t poc-client .