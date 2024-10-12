use anyhow::Result;
use my_redis_client::{log, redis_client::RedisClient, repl::Repl};

#[tokio::main]
async fn main() -> Result<()> {
    let mut redis_client = RedisClient::new(None, None).await;
    log!("Connected to Redis server");

    let mut repl = Repl::new();
    log!("Starting REPL");
    repl.run(&mut redis_client).await;
    Ok(())
}
