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
}
