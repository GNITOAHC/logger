use tonic::{transport::Server, Request, Response, Status};

use logger_service::logger_server::{Logger, LoggerServer};
use logger_service::{LogRequest, LogResponse};

mod config;
mod database;

pub mod logger_service {
    tonic::include_proto!("logger"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct MyLogger {
    helper: database::helper::Helper,
}

impl MyLogger {
    pub fn new(helper: database::helper::Helper) -> Self {
        Self { helper }
    }
}

#[tonic::async_trait]
impl Logger for MyLogger {
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<LogResponse>, Status> {
        println!("Got a request: {:?}", request);

        let entry = database::model::new_entry("name", "message", "200", "content");
        let res = &self.helper.new_log(entry).await;

        let response = LogResponse {
            message: format!("Hello, {}!", request.into_inner().message),
            status: "200".to_string(),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = config::config::new();

    let addr = format!("[::1]:{}", conf.port).parse()?;

    let db = database::database::new(&conf.mongo_url, &conf.database, &conf.collection).await;
    let helper = database::helper::new(db);

    let logger = MyLogger::new(helper);

    Server::builder()
        .add_service(LoggerServer::new(logger))
        .serve(addr)
        .await?;

    Ok(())
}
