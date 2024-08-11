use anyhow::Result;
use std::collections::HashSet;
use std::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct RedisClient {
    server_address: String,
    port: u16,
    stream: Option<TcpStream>,
    command_queue: Vec<String>,
    last_response: Option<String>,
}

impl RedisClient {
    pub fn new(addr: String, port: u16) -> RedisClient {
        let stream = TcpStream::connect(format!("{addr}:{port}")).unwrap();

        RedisClient {
            server_address: addr.to_string(),
            port: port,
            stream: Some(stream),
            command_queue: Vec::new(),
            last_response: None,
        }
    }

    pub async fn send_command(&mut self, command: String) -> Result<()> {
        self.stream.write_all(command).await.unwrap();
        Ok(())
    }
}
