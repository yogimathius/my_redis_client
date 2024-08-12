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

    pub fn run(&mut self, redis_client: &mut RedisClient) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer.push_str(&input);
            if self.buffer.ends_with("\n") {
                self.handle_input(redis_client);
            }
        }
    }

    pub async fn handle_input(&mut self, redis_client: &mut RedisClient) {
        let input = self.buffer.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            self.buffer.clear();
            return;
        }
        let command = parts[0];
        let args: Vec<String> = parts[1..].to_vec().iter().map(|s| s.to_string()).collect();

        if let Some(command_info) = COMMANDS.get(command) {
            let _ = command_info.validate_args(args.clone());
        } else {
            eprintln!("Invalid command: {}", command);
            std::process::exit(1);
        }

        let _ = redis_client.send_command(command.to_string(), args).await;
        self.buffer.clear();
    }
}
