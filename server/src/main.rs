mod poc;

use async_stream;
use futures::{Stream, StreamExt};
use poc::simple_request::Request::{JobRegisterRequest, OrderUpdateRequest};
use poc::{
    simple_connect_server::SimpleConnect, simple_connect_server::SimpleConnectServer,
    JobRegisterResponse, OrderUpdateResponse,
};
use poc::{simple_response, SimpleRequest, SimpleResponse};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
#[derive(Default)]
pub struct MockManagerService {
    //if jobs.get(job_id) is empty, then go and fetch the data from the db
    jobs: Arc<RwLock<HashMap<String, Vec<poc::OrderUpdateRequest>>>>,
    // no need to keep track of the inference channel for now
    manager_id: Arc<RwLock<String>>,
}

#[tonic::async_trait]
impl SimpleConnect for MockManagerService {
    type SimpleConnectStream =
        Pin<Box<dyn Stream<Item = Result<SimpleResponse, Status>> + Send + Sync + 'static>>;
    async fn simple_connect(
        &self,
        request: Request<tonic::Streaming<SimpleRequest>>,
    ) -> Result<Response<Self::SimpleConnectStream>, Status> {
        let manager_id = self.manager_id.clone();
        let jobs = self.jobs.clone();
        let (tx_endpoint, mut rx_endpoint) = mpsc::unbounded_channel();
        let tx_endpoint_in_request_handler = tx_endpoint.clone();
        let mut stream = request.into_inner();
        tokio::spawn(async move {
            while let Some(req) = stream.next().await {
                let unwrapped = req.unwrap();
                println!("request coming from client_id : {:?}", unwrapped.user_id);
        
                match unwrapped.request.unwrap() {
                    JobRegisterRequest(job_register_request) => {
                        let job_id = Uuid::new_v4();
                        println!("this is the new job id: {}", job_id);
                        let job_id = job_id.to_string();
                        jobs.write().await.insert(job_id.clone(), Vec::new());
                        let manager_id = manager_id.read().await; 
                        let res = SimpleResponse {
                            job_id,
                            manager_id: manager_id.to_string(),
                            response: Some(simple_response::Response::JobRegisterResponse(
                                JobRegisterResponse { status: true },
                            )),
                        };
                        tx_endpoint_in_request_handler.send(res);
                    }
                    OrderUpdateRequest(order_update_request) => {
                        let job_id = order_update_request.job_id.clone();
                        jobs.write()
                            .await
                            .entry(job_id.clone())
                            .or_default()
                            .push(order_update_request.clone());
                        //add all the values from the update
                        let mut sum = 0;
                        let temp = jobs.read().await.get(&job_id).unwrap()
                        .iter().map(|order| sum += order.value);
                        let manager_id = manager_id.read().await; 
                        let res = SimpleResponse {
                            job_id,
                            manager_id: manager_id.to_string(), 
                            response: Some(simple_response::Response::OrderUpdateResponse(
                                OrderUpdateResponse { sum },
                            )),
                        };
                        tx_endpoint_in_request_handler.send(res);
                    }
                }
            }
        });

        let output = async_stream::try_stream! {
            while let Some(res) = rx_endpoint.next().await {
            yield res;
            }
        };
        Ok(Response::new(Box::pin(output) as Self::SimpleConnectStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: ADD cassandra for a full POC
    //database should be something just <job_id, orderupdate for now>
    //later do think about including initial jobregisterequest

    let my_uuid = Uuid::new_v4();
    println!("i'm a manager, and this is my id: {}", my_uuid);
    let addr = "0.0.0.0:50051".parse()?;
    println!("{:?}", addr);
    //create the service

    let register_service = MockManagerService {
        jobs: Arc::new(Default::default()),
        manager_id: Arc::new(RwLock::new(my_uuid.to_string())),
    };
    Server::builder()
        .add_service(SimpleConnectServer::new(register_service))
        .serve(addr)
        .await?;

    Ok(())
}
