	
docker-build-server:
	cd server && docker build -t poc_server .

docker-build-client:
	cd client && docker build -t poc_client .