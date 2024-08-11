use std::net::TcpStream;

pub struct RedisClient {
    server_address: String,
    port: u16,
    tcp_stream: Option<TcpStream>,
    command_queue: Vec<String>,
    last_response: Option<String>,
}

impl RedisClient {
    pub fn new(addr: String, port: u16) -> RedisClient {
        let stream = TcpStream::connect(format!("{addr}:{port}")).unwrap();

        RedisClient {
            server_address: addr.to_string(),
            port: port,
            tcp_stream: Some(stream),
            command_queue: Vec::new(),
            last_response: None,
        }
    }
}
