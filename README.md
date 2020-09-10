# Introduction

This repo is to test the compatibility of k8s + grpc + istio + cassandra

k8s + grpc combination have following know problems

https://kubernetes.io/blog/2018/11/07/grpc-load-balancing-on-kubernetes-without-tears/

This repo is to test these problems and find ways to go around it. 

## Spec

Client: grpc client with five fake "Users" sending messages to the server 

Server: grpc server

Fluentd has been added for monitoring + logging

## Steps to test k8s+ grpc + built in LoadBalancer

0. [Optional] configure the namespace

```
kubectl config set-context --current --namespace=poc
```

1. build docker images for client and server 

```
make docker-build-server
make docker-build-client
```

2. Then create a namespace for poc
```
kubectl apply -f namespace.yaml
```
3. Then create a service 
```
kubectl apply -f services.yaml 
```
4. Then deploy the images
```
kubectl apply -f server-deployment.yaml
kubectl apply -f client-deployment.yaml 
```
5. Then check if everything's running by looking at logs for each server pods 
```
kubectl logs -l name=poc-server
```

## Steps to test k8s+ grpc + Istio
