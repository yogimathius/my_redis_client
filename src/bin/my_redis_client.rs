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

    let formatted_command = format!("{} {}", command, parameters.join(" "));

    let addr = "localhost".to_string();
    let port = 6739;
    let mut redis_client = RedisClient::new(addr, port).await;

    let _ = redis_client.send_command(formatted_command).await;
    Ok(())
}
