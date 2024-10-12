use crate::{
    command_info::{COMMANDS, DEPRECATED_COMMANDS},
    log,
    redis_client::RedisClient,
};

pub struct Repl {
    buffer: String,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            buffer: String::new(),
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
        let input = self.buffer.trim();
        log!("Handling input: {}", input);
        let parts: Vec<&str> = input.split_whitespace().collect();
        log!("split input into parts: {:?}", parts);
        if parts.is_empty() {
            self.buffer.clear();
            return;
        }
        let command = parts[0].to_uppercase();
        let args: Vec<String> = parts[1..].to_vec().iter().map(|s| s.to_string()).collect();

        if let Some(new_command) = DEPRECATED_COMMANDS.get(command.as_str()) {
            log!(
                "Warning: Command '{}' is deprecated. Use '{}' instead.",
                command,
                new_command
            );
        } else {
            self.process_command(redis_client, command, args).await;
        }

        self.buffer.clear();
    }

    pub async fn process_command(
        &mut self,
        redis_client: &mut RedisClient,
        command: String,
        args: Vec<String>,
    ) {
        if let Some(command_info) = COMMANDS.get(command.to_uppercase().as_str()) {
            match command_info.validate_and_transform_args(args) {
                Ok(transformed_args) => {
                    log!("Sending command: {} {:?}", command, transformed_args);
                    match redis_client
                        .send_command(command.to_string(), transformed_args)
                        .await
                    {
                        Ok(response) => {
                            log!("Response: {}", response);
                        }
                        Err(e) => {
                            log!("Error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log!("Invalid arguments: {}", e);
                }
            }
        } else {
            log!("Invalid command: {}", command);
        }
    }
}
