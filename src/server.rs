use ::clap::Parser;
use api::boolean_service_server::{BooleanService, BooleanServiceServer};
use api::{BooleanRequest, BooleanResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug, Default)]
pub struct Boolean {}

#[tonic::async_trait]
impl BooleanService for Boolean {
    async fn and(&self, request: Request<BooleanRequest>) -> Result<Response<BooleanResponse>, Status> {
        let request = request.into_inner();
        println!("Got AND request: {request:?}");

        Ok(Response::new(BooleanResponse {
            value: request.values.iter().all(|&v| v),
        }))
    }

    async fn or(&self, request: Request<BooleanRequest>) -> Result<Response<BooleanResponse>, Status> {
        let request = request.into_inner();
        println!("Got OR request: {request:?}");

        Ok(Response::new(BooleanResponse {
            value: request.values.iter().any(|&v| v),
        }))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "bool-server - a simple bool microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50123")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let boolean = Boolean::default();

    println!("Boolean server listening on {addr}");

    Server::builder().add_service(BooleanServiceServer::new(boolean)).serve(addr).await?;

    Ok(())
}
