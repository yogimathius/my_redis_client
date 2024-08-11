use my_redis_client::redis_client::RedisClient;

fn main() {
    let command = std::env::args().nth(1).expect("Usage: my_ping <command>");
    let parameters: Vec<String> = std::env::args().skip(2).collect();

    if !VALID_COMMANDS.contains(command.as_str()) {
        eprintln!("Invalid command: {}", command);
        std::process::exit(1);
    }

    let formatted_command = format!("{} {}", command, parameters.join(" "));

    let addr = "localhost".to_string();
    let port = 6739;
    let redis_client = RedisClient::new(addr, port);

    redis_client.send_command(formatted_command)
}
