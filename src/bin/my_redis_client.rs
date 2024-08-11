use my_redis_client::redis_client::RedisClient;

fn main() {
    let command = std::env::args().nth(1).expect("Usage: my_ping <command>");
    let parameters: Vec<String> = std::env::args().skip(2).collect();

    let addr = "localhost".to_string();
    let port = 6739;
    let redis_client = RedisClient::new(addr, port);
}
