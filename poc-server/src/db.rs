use std::collections::HashMap;

use azure_sdk_cosmos::prelude::*;
use azure_sdk_cosmos::responses::ListDocumentsResponse;
use azure_sdk_cosmos::responses::QueryDocumentsResponse;
use azure_sdk_cosmos::Database;
use azure_sdk_cosmos::{clients::DefaultCosmosUri, DatabaseClient};
use serde::{Deserialize, Serialize};

//TODO choose the right partitition key. For now, it's just itemid
//TODO write trigger functions for state retrieval and job ids retrieval

// all events for a single jobid
#[derive(Serialize, Deserialize)]
pub enum AllEventStruct {}
//each insert is of this form
//need some other logic for reading the document
#[derive(Serialize, Deserialize)]
pub enum EventStruct {}
// db interfaces
#[derive(Debug, Clone)]
pub struct ManagerDatabase<'a> {
    pub client: CosmosStruct<'a, DefaultCosmosUri>,
    pub name: String,
}

impl<'a> ManagerDatabase<'a> {
    async fn get_all_user_ids(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut user_list: Vec<String> = Vec::default();
        let temp = self
            .client
            .clone()
            .into_database_client(self.name.clone())
            .list_collections()
            .execute()
            .await?;
        for collection in temp.collections {
            user_list.push(collection.id);
        }
        return Ok(user_list);
    }

    async fn get_user_data(
        &self,
        user_id: &str,
    ) -> Result<Vec<AllEventStruct>, Box<dyn std::error::Error>> {
        let mut user_data: Vec<AllEventStruct> = Vec::default();
        let temp = self
            .client
            .clone()
            .into_database_client(self.name.clone())
            .into_collection_client(user_id)
            .list_documents()
            .execute()
            .await?;

        for doc in temp.documents {
            user_data.push(doc.document);
        }

        return Ok(user_data);
    }
    async fn get_user_jobids(
        &self,
        user_id: &str,
        job_id: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut user_jobids: Vec<String> = Vec::default();

        let temp = self
            .client
            .clone()
            .into_database_client(self.name.clone())
            .into_collection_client(user_id)
            .list_documents()
            .execute::<AllEventStruct>()
            .await?;

        for doc in temp.documents {
            //TODO: write some trigger function
            //user_data.push(doc.document);
        }

        return Ok(user_jobids);
    }
    //should this be caculated here?
    async fn get_job_data(
        &self,
        user_id: &str,
        job_id: &str,
    ) -> Result<Vec<EventStruct>, Box<dyn std::error::Error>> {
        let mut user_jobids: Vec<EventStruct> = Vec::default();

        let temp = self
            .client
            .clone()
            .into_database_client(self.name.clone())
            .into_collection_client(user_id)
            //TODO choose the right parition key
            // for write heavy db, item id('_rid') is a good choice
            // job id could be good too but not all events have that
            // Should be noted that PartitionKey isn't required for collections under 10GB
            .into_document_client(job_id, "_rid".into())
            .get_document()
            .execute::<AllEventStruct>()
            .await?;

        //somehow deserialize the document
        return Ok(user_jobids);
    }

    async fn add_event(
        &self,
        user_id: &str,
        job_id: &str,
        even: EventStruct,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        //TODO Use a user defined function for appending a single item to the document
        // let temp = self
        //     .client
        //     .clone()
        //     .into_database_client(self.name.clone())
        //     .into_collection_client(user_id)
        //     //TODO choose the right parition key
        //     // for write heavy db, item id('_rid') is a good choice
        //     // job id could be good too but not all events have that
        //     // Should be noted that PartitionKey isn't required for collections under 10GB
        //     .into_document_client(job_id, "_rid".into())
        //     .get_document()
        //     .
        //     .execute()
        //     .await?;
        Ok(true)
    }

    //do we need this
    async fn create_user(&self) {}
}

pub async fn connect_to_db() -> Result<ManagerDatabase<'static>, Box<dyn std::error::Error>> {
    let account = String::from("account goes here");
    let master_key = String::from("master key goes here");
    let database_name: String = String::from("manager");
    let authorization_token = AuthorizationToken::new_master(&master_key)?;
    let client = ClientBuilder::new(account, authorization_token)?;
    let temp = ManagerDatabase {
        client,
        name: database_name,
    };

    return Ok(temp);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = String::from("account goes here");
    let master_key = String::from("master key goes here");

    //     let account = std::env::args()
    //      .nth(1)
    //      .expect("pass the CosmosDB account name as first parameter!");
    //  let master_key = std::env::args()
    //      .nth(2)
    //      .expect("pass the CosmosDB master key as second parameter!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;
    let client = ClientBuilder::new(&account, authorization_token)?;
    let databases = client.list_databases().execute().await?;

    println!("{:#?}", databases);

    // create a new db if we encounter new user id

    let temp = client
        .create_database()
        .with_database_name(&"louis") // we must specify a name!!!
        .execute()
        .await?;

    let hi = client.into_database_client("louis");
    let dd = hi.into_collection_client("jobid");

    let response: ListDocumentsResponse<serde_json::Value> = dd.list_documents().execute().await?;

    Ok(())
}
