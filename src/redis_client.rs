use anyhow::Result;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::models::value::Value;

pub struct RedisClient {
    pub server_address: String,
    pub port: u16,
    pub stream: Option<TcpStream>,
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
            stream: Some(stream),
            command_queue: Vec::new(),
            last_response: None,
        }
    }

    pub async fn send_command(&mut self, command: String, params: Vec<String>) -> Result<String> {
        let mut msg = vec![Value::BulkString(command)];
        msg.extend(params.into_iter().map(Value::BulkString));

        println!("Sending msg: {:?}", msg);
        let payload = Value::Array(msg).serialize();

        if let Some(stream) = &mut self.stream {
            stream.write_all(&payload).await?;
            let response = self.read_response().await?;
            let parsed_response = Value::deserialize(&response)?;
            Ok(parsed_response.to_string())
        } else {
            Err(anyhow::anyhow!("Not connected to Redis server"))
        }
    }

    pub async fn read_response(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = [0; 512];
        if let Some(stream) = &mut self.stream {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Connection closed by the server",
                ));
            }
            Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Not connected to Redis server",
            ))
        }
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream.shutdown().await?;
        }
        Ok(())
    }
}
