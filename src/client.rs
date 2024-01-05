use ::clap::Parser;
use api::boolean_service_client::BooleanServiceClient;
use api::BooleanOperator;
use api::BooleanRequest;

pub mod api {
    tonic::include_proto!("api");
}

impl ::clap::ValueEnum for BooleanOperator {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::And, Self::Or]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::And => Some("and".into()),
            Self::Or => Some("or".into()),
        }
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "bool-client - a simple CLI to send bool expressions to a server", long_about = None)]
struct ClientCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,

    #[arg(short = 'p', long = "port", default_value = "50123")]
    port: u16,

    #[arg(value_parser = clap::builder::EnumValueParser::<BooleanOperator>::new())]
    op: BooleanOperator,

    #[arg(action = clap::builder::ArgAction::Set)]
    values: Vec<bool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ClientCli::parse();
    let mut client = BooleanServiceClient::connect(format!("http://{}:{}", cli.server, cli.port)).await?;

    let request = tonic::Request::new(BooleanRequest {
        op: cli.op as i32,
        values: cli.values,
    });

    let response = match cli.op {
        BooleanOperator::And => client.and(request).await?,
        BooleanOperator::Or => client.or(request).await?,
    };

    println!("RESULT={:?}", response.into_inner().value);
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
