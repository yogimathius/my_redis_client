use my_redis_client::redis_client::RedisClient;

fn main() {
    let addr = "localhost".to_string();
    let port = 6739;
    let redis_client = RedisClient::new(addr, port);
}
