use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::models::value::Value;

pub struct RedisClient {
    pub server_address: String,
    pub port: u16,
    pub stream: TcpStream,
    pub command_queue: Vec<String>,
    pub last_response: Option<String>,
}

impl RedisClient {
    pub async fn new(addr: Option<String>, port: Option<u16>) -> RedisClient {
        let default_addr = addr.unwrap_or("localhost".to_string());
        let default_port = port.unwrap_or(6379);

        let stream = TcpStream::connect(format!("{default_addr}:{default_port}"))
            .await
            .unwrap();
        println!("connecting");
        RedisClient {
            server_address: default_addr,
            port: default_port,
            stream: stream,
            command_queue: Vec::new(),
            last_response: None,
        }
    }

    pub async fn send_command(&mut self, command: String, params: Vec<String>) -> Result<()> {
        let mut msg = vec![Value::BulkString(String::from(command))];
        msg.extend(params.into_iter().map(Value::BulkString));

        println!("sending msg: {:?}", msg);
        let payload = Value::Array(msg);
        self.stream
            .write_all(payload.serialize().as_bytes())
            .await
            .unwrap();
        Ok(())
    }

    pub async fn read_response(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = [0; 512];
        let n = self.stream.read(&mut buffer).await?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Connection closed by the server",
            ));
        }
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }
}
