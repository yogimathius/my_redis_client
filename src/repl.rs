use crate::{command_info::COMMANDS, redis_client::RedisClient};

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
        println!("Welcome to the Redis REPL");
        loop {
            println!("> ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer.push_str(&input);
            println!("added input to buffer: {}", self.buffer.trim());
            if self.buffer.ends_with("\n") {
                self.handle_input(redis_client).await;
            }
        }
    }

    pub async fn handle_input(&mut self, redis_client: &mut RedisClient) {
        let input = self.buffer.trim();
        println!("Handling input: {}", input);
        let parts: Vec<&str> = input.split_whitespace().collect();
        println!("split input into parts: {:?}", parts);
        if parts.is_empty() {
            self.buffer.clear();
            return;
        }
        let command = parts[0];
        let args: Vec<String> = parts[1..].to_vec().iter().map(|s| s.to_string()).collect();

        if let Some(command_info) = COMMANDS.get(command.to_uppercase().as_str()) {
            let _ = command_info.validate_args(args.clone());
        } else {
            eprintln!("Invalid command: {}", command);
            self.buffer.clear();
            return;
        }

        println!("Sending command: {} {:?}", command, args);

        match redis_client.send_command(command.to_string(), args).await {
            Ok(response) => {
                println!("Response: {}", response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        self.buffer.clear();
    }
}
