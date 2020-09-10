# Introduction

This is the poc version of axe endpoint and manager on kubernetes. 

## Spec


## TODO 

[ ] use cassandra

[ ] grpc pod connection check

[ ] 복구 플랜 with cassandra

[ ] attach fluentd

## Steps to test POC


1. build docker images for client and server 

make docker-build-server
make docker-build-client


2. Then create a namespace for poc

kubectl apply -f namespace.yaml

3. Then deploy the images

kubectl apply -f deployments.yaml 

4. Then create a service 

kubectl apply -f services.yaml 

5. Then check if everything's running by looking at logs for each server pods 

kubectl logs -l name=poc-server

6. 


## Clean up POC 


