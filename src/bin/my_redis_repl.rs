use anyhow::Result;
use my_redis_client::{redis_client::RedisClient, repl::Repl};

#[tokio::main]
async fn main() -> Result<()> {
    let mut redis_client = RedisClient::new(None, None).await;

    let mut repl = Repl::new();
    repl.run(&mut redis_client);
    Ok(())
}
