#[cfg(test)]
mod tests {
    use my_redis_client::redis_client::RedisClient;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio::task;

    pub async fn setup_mock_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn a task to accept the connection
        task::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            // You can add more logic here to handle the mock server behavior
            let _ = socket;
        });

        addr.to_string()
    }

    #[tokio::test]
    async fn test_redis_client() {
        let addr = setup_mock_server().await;

        let mut client = RedisClient::new(
            Some(addr.split(':').next().unwrap().to_string()),
            Some(addr.split(':').nth(1).unwrap().parse().unwrap()),
        )
        .await;
        assert!(client.port == addr.split(':').nth(1).unwrap().parse().unwrap());
        assert!(client.server_address == addr.split(':').next().unwrap().to_string());
        assert!(client.command_queue.is_empty());
        assert!(client.last_response.is_none());

        let result = client.stream.write(&[]).await;
        assert!(result.is_ok(), "Expected stream to be in a good state");
    }
}
