use ::clap::Parser;
use computemicro::service_client::ServiceClient;
use computemicro::{ComputeOperator, ComputeRequest};

pub mod baseresponse {
    tonic::include_proto!("baseresponse");
}
pub mod computemicro {
    tonic::include_proto!("computemicro");
}

impl ::clap::ValueEnum for ComputeOperator {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::And, Self::Or]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::And => Some("and".into()),
            Self::Or => Some("or".into()),
            _ => None,
        }
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "compute-client - a simple CLI to send bool expressions to a server", long_about = None)]
struct ClientCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,

    #[arg(short = 'p', long = "port", default_value = "50123")]
    port: u16,

    #[arg(value_parser = clap::builder::EnumValueParser::<ComputeOperator>::new())]
    op: ComputeOperator,

    #[arg(action = clap::builder::ArgAction::Set)]
    values: Vec<bool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ClientCli::parse();
    let mut client = ServiceClient::connect(format!("http://{}:{}", cli.server, cli.port)).await?;

    let response = client
        .compute(tonic::Request::new(ComputeRequest {
            op: cli.op as i32,
            inputs: cli.values,
        }))
        .await?;

    println!("RESULT={:?}", response.into_inner().output);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
