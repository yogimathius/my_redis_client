use anyhow::Result;
use my_redis_client::{command_info::CommandInfo, log, redis_client::RedisClient};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "my_redis_cli", about = "A Redis CLI client")]
struct Cli {
    command: String,

    #[structopt(parse(from_str))]
    args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Cli { command, args } = Cli::from_args();
    let command = command.to_uppercase();

    let (validated_command, transformed_args) = CommandInfo::new(&command, args)?;

    let mut redis_client = RedisClient::new(None, None).await;
    let result = redis_client
        .send_command(validated_command, transformed_args)
        .await?;

    log!("{:?}", result);
    Ok(())
}
