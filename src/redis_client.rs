use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub struct RedisClient {
    server_address: String,
    port: u16,
    stream: TcpStream,
    command_queue: Vec<String>,
    last_response: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    Array(Vec<Value>),
    NullBulkString,
}

impl Value {
    pub fn serialize(self) -> String {
        match self {
            Value::Array(values) => {
                let mut serialized = format!("*{}\r\n", values.len());
                for value in values {
                    serialized.push_str(&value.serialize());
                }

                serialized
            }
            Value::SimpleString(s) => format!("+{}\r\n", s),
            Value::BulkString(s) => format!("${}\r\n{}\r\n", s.chars().count(), s),
            Value::NullBulkString => format!("$-1\r\n"),
        }
    }
}

impl RedisClient {
    pub async fn new(addr: String, port: u16) -> RedisClient {
        let stream = TcpStream::connect(format!("{addr}:{port}")).await.unwrap();

        RedisClient {
            server_address: addr.to_string(),
            port: port,
            stream: stream,
            command_queue: Vec::new(),
            last_response: None,
        }
    }

    pub async fn send_command(&mut self, command: String) -> Result<()> {
        let msg = vec![Value::BulkString(String::from(command))];
        let payload = Value::Array(msg);
        self.stream
            .write_all(payload.serialize().as_bytes())
            .await
            .unwrap();
        Ok(())
    }
}
