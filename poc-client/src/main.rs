mod poc;
use crate::poc::simple_response::Response::JobRegisterResponse;
use crate::poc::simple_response::Response::OrderUpdateResponse;
use poc::{simple_connect_client::SimpleConnectClient, JobRegisterRequest, OrderUpdateRequest};
use poc::{simple_request, SimpleRequest};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::{thread, time};
use tokio::sync::{mpsc, RwLock};
use tonic::{transport::Channel, Request};
use uuid::Uuid;
//pub type UserData = Arc<RwLock<HashMap<String, Vec<String>>>>;
pub type RClient = Arc<RwLock<SimpleConnectClient<Channel>>>;

// pub type JobIDs = Arc<RwLock<String>>;
/*
    Can be reused as a mock client in the future
*/
//TODO make this into stream request not just one time thing
#[tokio::main]
async fn main() {
    // 1. create X number of users(default is 1)
    let number_of_users = env::var("user_number").unwrap_or(String::from("5"));
    let number_of_users = number_of_users.parse::<u32>().unwrap_or(1);

    println!("This is number of fake users: {:?}", number_of_users);

    // 1-1 accept the the server port
    env::vars().for_each(|temp| println!("{}, {}", temp.0, temp.1));
    let server_endpoint = env::var("POC_SERVER_PORT").unwrap_or("tcp://0.0.0.0:50051".into());
    println!("this is the server endpoint: {}", server_endpoint);
    // 2. create a task for X number of users each doing own thing
    let client = Arc::new(RwLock::new(
        SimpleConnectClient::connect(server_endpoint).await.unwrap(),
    ));
    let client2 = client.clone();

    let client3 = client.clone();

    let client4 = client.clone();
    let client5 = client.clone();

    //더럽습니다. 죄송합니다. cannot use values that have been moved 싫습니다.
    tokio::spawn(async move {
        tokio::task::spawn(async move {
            println!("user1");
            handle(client2).await;
        });
        thread::sleep(time::Duration::from_secs(1));
        tokio::task::spawn(async move {
            println!("user2");

            handle(client3).await;
        });
        thread::sleep(time::Duration::from_secs(1));
        tokio::task::spawn(async move {
            println!("user3");

            handle(client4).await;
        });
        thread::sleep(time::Duration::from_secs(1));

        tokio::task::spawn(async move {
            println!("user4");

            handle(client5).await;
        });
        thread::sleep(time::Duration::from_secs(1));

        tokio::task::spawn(async move {
            println!("user5");
            handle(client.clone()).await;
        })
        .await;
    })
    .await;

    // tokio::task::spawn(async move {
    //     println!("hi1");
    //     handle(client.clone()).await;
    // })
    // .await;

    // tokio::task::spawn(async move {
    //     println!("hi2");

    //     handle(client2).await;
    // })
    // .await;

    // tokio::task::spawn(async move {
    //     handle(client2).await;
    // });

    // for n in 1..number_of_users + 1 {
    //     println!("------------------- user number: {:?}", n);
    //     tokio::task::spawn(async move {
    //         handle(&client).await;
    //     });
    // }
}

async fn handle(client: RClient) {
    // we need each task to remain 'active'

    let (message_sender, message_receiver) = mpsc::unbounded_channel();
    let user_id = Uuid::new_v4().to_string();
    println!("This is a new user id: {:?}", user_id);
    let mut job_id = String::from("hi");
    //let mut job_id = Arc::new(RwLock::new(String::from("default")));
    let job_register_request = JobRegisterRequest { register: true };
    let first_request = SimpleRequest {
        user_id: user_id.clone(),
        request: Some(simple_request::Request::JobRegisterRequest(
            job_register_request,
        )),
    };
    message_sender.send(first_request);

    let response = client
        .write()
        .await
        .simple_connect(Request::new(message_receiver))
        .await;

    match response {
        Ok(response) => {
            let mut inbound = response.into_inner();

            while let Some(res) = inbound.message().await.unwrap() {
                match res.response.unwrap() {
                    JobRegisterResponse(job_register_response) => {
                        if job_register_response.status {
                            println!(
                                "Job has been registerd: {:?} by the manager: {:?}",
                                res.job_id, res.manager_id
                            );
                            std::mem::replace(&mut job_id, res.job_id);
                            break;
                        }
                    }
                    OrderUpdateResponse(order_update_response) => {
                        let order_update_response = order_update_response.sum;
                        println!(
                            "Job id: {:?}, manager id: {:?}, sum:{:?}",
                            res.job_id, res.manager_id, order_update_response
                        );
                    }
                }
            }
        }
        Err(err) => println!("print the status:{:?}", err.code()),
    }

    let (new_message_sender, new_message_receiver) = mpsc::unbounded_channel();

    thread::spawn(move || {
        //loop everything below
        loop {
            //message sending part
            thread::sleep(time::Duration::from_secs(1));
            let order_update_request = OrderUpdateRequest {
                job_id: job_id.clone(),
                value: 3,
            };
            let second_request = SimpleRequest {
                user_id: user_id.clone(),
                request: Some(simple_request::Request::OrderUpdateRequest(
                    order_update_request,
                )),
            };
            println!("sending a order update message");
            new_message_sender.send(second_request);
        }
    });

    let response = client
        .write()
        .await
        .simple_connect(Request::new(new_message_receiver))
        .await;

    match response {
        Ok(response) => {
            let mut inbound = response.into_inner();

            while let Some(res) = inbound.message().await.unwrap() {
                match res.response.unwrap() {
                    JobRegisterResponse(job_register_response) => {
                        if job_register_response.status {
                            println!(
                                "Job has been registerd: {:?} by the manager: {:?}",
                                res.job_id, res.manager_id
                            );
                        }
                    }
                    OrderUpdateResponse(order_update_response) => {
                        let order_update_response = order_update_response.sum;
                        println!(
                            "Job id: {:?}, manager id: {:?}, sum:{:?}",
                            res.job_id, res.manager_id, order_update_response
                        );
                    }
                }
            }
            println!("escapedddd");
        }
        Err(err) => println!("print the status:{:?}", err.code()),
    }
}
