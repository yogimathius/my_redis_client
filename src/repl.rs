use crate::{command_info::CommandInfo, log, models::value::Value, redis_client::RedisClient};
use tokio::io::{self};

pub struct Repl {
    buffer: String,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            buffer: String::with_capacity(1024),
        }
    }

    pub async fn run(&mut self, redis_client: &mut RedisClient) {
        log!("Welcome to the Redis REPL");

        loop {
            log!("> ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer.push_str(&input);
            log!("added input to buffer: {}", self.buffer.trim());
            if self.buffer.ends_with("\n") {
                self.handle_input(redis_client).await;
            }
        }
    }

    pub async fn handle_input(&mut self, redis_client: &mut RedisClient) {
        let input = std::mem::take(&mut self.buffer);
        let input = input.trim();
        log!("Handling input: {:?}", input);
        if input.is_empty() {
            return;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().map(|s| s.to_uppercase()).unwrap_or_default();
        let args: Vec<String> = parts.map(String::from).collect();

        match CommandInfo::new(&command, args) {
            Ok((validated_command, transformed_args)) => {
                self.process_command(redis_client, validated_command, transformed_args)
                    .await;
            }
            Err(e) => {
                log!("Error validating command: {}", e);
            }
        }
    }

    pub async fn process_command(
        &mut self,
        redis_client: &mut RedisClient,
        command: String,
        args: Vec<Value>,
    ) {
        log!("Sending command: {} {:?}", command, args);
        match redis_client.send_command(command.to_string(), args).await {
            Ok(response) => {
                log!("Response: {}", response);
            }
            Err(e) => {
                log!("Error: {}", e);
            }
        }
    }
}
