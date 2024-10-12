use anyhow::Result;
use my_redis_client::{command_info::COMMANDS, redis_client::RedisClient};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "my_redis_cli", about = "A Redis CLI client")]
struct Cli {
    /// Redis command to execute
    command: String,

    /// Arguments for the Redis command
    #[structopt(parse(from_str))]
    args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();

    if let Some(command_info) = COMMANDS.get(cli.command.as_str()) {
        let _ = command_info.validate_args(cli.args.clone());
    } else {
        println!("Invalid command: {}", cli.command);
        std::process::exit(1);
    }

    let mut redis_client = RedisClient::new(None, None).await;

    let result = redis_client.send_command(cli.command, cli.args).await?;
    println!("{:?}", result);
    Ok(())
}
