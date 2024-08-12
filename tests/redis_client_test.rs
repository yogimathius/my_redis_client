#[cfg(test)]
mod tests {
    use my_redis_client::redis_client::RedisClient;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio::task;

    #[tokio::test]
    async fn test_redis_client() {
        // Start a mock server
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn a task to accept the connection
        task::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            // You can add more logic here to handle the mock server behavior
            let _ = socket;
        });

        // Create the RedisClient and connect to the mock server
        let mut client = RedisClient::new(addr.ip().to_string(), addr.port()).await;
        assert!(client.port == addr.port());
        assert!(client.server_address == addr.ip().to_string());
        assert!(client.command_queue.is_empty());
        assert!(client.last_response.is_none());

        // Check if the stream is in a good state by attempting a non-blocking write
        let result = client.stream.write(&[]).await;
        assert!(result.is_ok(), "Expected stream to be in a good state");
    }
}
