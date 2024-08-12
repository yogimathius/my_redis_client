use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub struct RedisClient {
    pub server_address: String,
    pub port: u16,
    pub stream: TcpStream,
    pub command_queue: Vec<String>,
    pub last_response: Option<String>,
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
        println!("connecting");
        RedisClient {
            server_address: addr.to_string(),
            port: port,
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
}
