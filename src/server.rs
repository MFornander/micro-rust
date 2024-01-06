use ::clap::Parser;
use computemicro::service_server::{Service, ServiceServer};
use computemicro::{ComputeOperator, ComputeRequest, ComputeResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod baseresponse {
    tonic::include_proto!("baseresponse");
}

pub mod computemicro {
    tonic::include_proto!("computemicro");
}

#[derive(Debug, Default)]
pub struct ComputeMicro {}

#[tonic::async_trait]
impl Service for ComputeMicro {
    async fn compute(&self, request: Request<ComputeRequest>) -> Result<Response<ComputeResponse>, Status> {
        let request = request.into_inner();
        println!("Got request: {request:?}");

        let result: bool = match ComputeOperator::try_from(request.op) {
            Ok(ComputeOperator::And) => request.inputs.iter().all(|&v| v),
            Ok(ComputeOperator::Or) => request.inputs.iter().any(|&v| v),
            _ => return Err(Status::invalid_argument("invalid operator")),
        };

        Ok(Response::new(ComputeResponse { response: None, output: result }))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "compute-server - a simple bool microservice", long_about = None)]
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
    let compute = ComputeMicro::default();

    println!("Boolean server listening on {addr}");

    Server::builder().add_service(ServiceServer::new(compute)).serve(addr).await?;

    Ok(())
}
