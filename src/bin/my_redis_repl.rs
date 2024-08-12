use anyhow::Result;
use my_redis_client::{command_info::COMMANDS, redis_client::RedisClient};

#[tokio::main]
async fn main() -> Result<()> {
    let command = std::env::args().nth(1).expect("Usage: my_ping <command>");
    let parameters: Vec<String> = std::env::args().skip(2).collect();

    if let Some(command_info) = COMMANDS.get(command.as_str()) {
        let _ = command_info.validate_args(parameters.clone());
    } else {
        eprintln!("Invalid command: {}", command);
        std::process::exit(1);
    }

    let mut redis_client = RedisClient::new(None, None).await;

    let _ = redis_client.send_command(command, parameters).await;
    Ok(())
}
