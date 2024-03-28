use super::model;
use mongodb::{
    bson::{doc, Document},
    Client, Collection, Database,
};

#[derive(Debug)]
pub struct MyDatabase {
    client: Client,
    database: Database,
    collection: Collection<Document>,
}

impl MyDatabase {
    pub async fn insert_one(
        &self,
        log_schema: model::LogSchema,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("(database) Inserting: {:?}", log_schema);
        let doc = doc! {
            "name": log_schema.name,
            "message": log_schema.message,
            "status": log_schema.status,
            "content": log_schema.content,

        };
        let res = self.collection.insert_one(doc, None).await;
        println!("(database) Result: {:?}", res);
        Ok(())
    }
}

pub async fn new(uri: &str, db: &str, c: &str) -> MyDatabase {
    let client_res = Client::with_uri_str(uri).await;
    if client_res.is_err() {
        panic!("Failed to connect to the database");
    }
    let client = client_res.unwrap();
    println!("Connected to the database");
    let database = client.database(db);
    let collection = database.collection(c);
    MyDatabase {
        client,
        database,
        collection,
    }
}
