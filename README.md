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


1. Install istio on local minikube
https://istio.io/latest/docs/setup/getting-started/

2. Configure Istio services




## Useful libraries + Miscellaneous ideas

Istio

https://bcho.tistory.com/1293?category=731548
https://bcho.tistory.com/1295?category=731548

https://www.youtube.com/watch?v=1iyFq2VaL5Y

https://crates.io/crates/avro-rs

https://kubernetes.io/docs/concepts/services-networking/service/

Istio mesh microservices GUI tool
1. https://kiali.io/
2. https://github.com/kiali/kiali


Instead of making LB do the work, we could use grpc connetion pool from the client side(make rust library in the future)
1. [Grpc connection pool](https://github.com/processout/grpc-go-pool/blob/master/pool.go)

Database connection pool
1. https://github.com/sfackler/r2d2

Istio example
1. https://github.com/GoogleCloudPlatform/istio-samples/tree/master/sample-apps/grpc-greeter-go/manifests


https://www.cncf.io/projects/

Maybe for endpoint db

https://github.com/tikv/tikv


## Things to check for stability(FCAPS)

1. Fault management
