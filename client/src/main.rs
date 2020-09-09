mod poc;
use job_scheduler::{Job, JobScheduler};
use poc::{
    simple_connect_client::SimpleConnectClient, simple_request, JobRegisterRequest,
    OrderUpdateRequest, SimpleRequest,
};
use std::env;
use tonic::transport::Channel;
use uuid::Uuid;
//TODO make this into stream request not just one time thing
#[tokio::main]
async fn main() {
    let my_uuid = Uuid::new_v4();
    println!("This is my job id: {}", my_uuid);
    let server_endpoint = env::var("server_port").unwrap_or("tcp://0.0.0.0:50051".into());
    let channel = Channel::from_static(&server_endpoint).connect().await;
    match channel {
        Err(err) => {}
        Ok(channel) => {
            let mut client = SimpleConnectClient::new(channel);
            let job_register_request = JobRegisterRequest { register: true };
            let firstConnectionRequest = SimpleRequest {
                job_id: my_uuid.to_string(),
                request: Some(simple_request::Request::JobRegisterRequest(
                    job_register_request,
                )),
            };
            let connection_response = client.simple_connect(firstConnectionRequest).await;
            match connection_response {
                Err(status) => {}
                Ok(response) => {
                    let mut inbound = response.into_inner();

                    loop {
                        match inbound.message().await {
                            Ok(res) => match res {
                                Some(result) => {
                                    println!("this is the response :{:?}", result);
                                    match result.response.unwrap() {
                                        poc::simple_response::Response::RegisterResponse(
                                            register_response,
                                        ) => {
                                            if register_response.status {
                                                send_update(client, my_uuid);
                                            }
                                        }
                                        poc::simple_response::Response::UpdateResponse(
                                            update_response,
                                        ) => {
                                            println!(
                                                "this is the updated sum: {:?}",
                                                update_response.sum
                                            );
                                        }
                                    }
                                }
                                None => {}
                            },
                            Err(_) => {}
                        }
                    }
                }
            }
        }
    }
}

fn send_update(client: SimpleConnectClient<Channel>, my_uuid: Uuid) {
    //for each second, keep sending these requests

    let mut sched = JobScheduler::new();
    sched.add(Job::new("1 * * * * *".parse().unwrap(), || {
        println!("I get executed every 1 seconds!");
        let order_update_request = OrderUpdateRequest { value: 2 };
        let request = SimpleRequest {
            job_id: my_uuid.to_string(),
            request: Some(simple_request::Request::OrderUpdateRequest(
                order_update_request,
            )),
        };
        client.simple_connect(request);
    }));
}
