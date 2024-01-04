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
    async fn and(
        &self,
        request: Request<BooleanRequest>,
    ) -> Result<Response<BooleanResponse>, Status> {
        for value in request.into_inner().values {
            match value {
                true => continue,
                false => return Ok(Response::new(BooleanResponse { value: false })),
            }
        }
        Ok(Response::new(BooleanResponse { value: true }))
    }

    async fn or(
        &self,
        request: Request<BooleanRequest>,
    ) -> Result<Response<BooleanResponse>, Status> {
        for value in request.into_inner().values {
            match value {
                false => continue,
                true => return Ok(Response::new(BooleanResponse { value: true })),
            }
        }
        Ok(Response::new(BooleanResponse { value: false }))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "bool-server - a simple bool microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let boolean = Boolean::default();

    println!("Bool server listening on {}", addr);

    Server::builder()
        .add_service(BooleanServiceServer::new(boolean))
        .serve(addr)
        .await?;

    Ok(())
}
