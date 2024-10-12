use anyhow::{anyhow, Result};
use my_redis_client::{
    command_info::{COMMANDS, DEPRECATED_COMMANDS},
    log,
    redis_client::RedisClient,
};
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

    let command = cli.command.to_uppercase();

    if let Some(new_command) = DEPRECATED_COMMANDS.get(command.as_str()) {
        log!(
            "Warning: Command '{}' is deprecated. Use '{}' instead.",
            command,
            new_command
        );
        return Err(anyhow!(
            "Command '{}' is deprecated. Use '{}' instead.",
            command,
            new_command
        ));
    }

    let transformed_args = if let Some(command_info) = COMMANDS.get(command.as_str()) {
        log!("Validating command: {:?}", command);
        command_info.validate_and_transform_args(cli.args)?
    } else {
        return Err(anyhow!("Invalid command: {}", cli.command));
    };

    let mut redis_client = RedisClient::new(None, None).await;

    let result = redis_client.send_command(command, transformed_args).await?;
    log!("{:?}", result);
    Ok(())
}
