use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

lazy_static! {
    pub static ref VALID_COMMANDS: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("quit");
        m.insert("echo");
        m.insert("ping");
        m.insert("flushall");
        m.insert("info");
        m.insert("set");
        m.insert("get");
        m.insert("key");
        m.insert("type");
        m.insert("del");
        m.insert("unlink");
        m.insert("expire");
        m.insert("rename");
        m.insert("llen");
        m.insert("lrem");
        m.insert("lindex");
        m.insert("lpop");
        m.insert("rpop");
        m.insert("lpush");
        m.insert("rpush");
        m.insert("lset");
        m.insert("hget");
        m.insert("hexists");
        m.insert("hdel");
        m.insert("hgetall");
        m.insert("hkeys");
        m.insert("hlen");
        m.insert("hmset");
        m.insert("hset");
        m.insert("hvals");
        m
    };
}

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
