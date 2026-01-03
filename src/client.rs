use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

async fn connect_with_retry(max_attempts: u32) -> Result<TcpStream, std::io::Error> {
    let mut last_error = None;

    for _ in 0..max_attempts {
        match TcpStream::connect("127.0.0.1:6379").await {
            Ok(stream) => return Ok(stream),
            Err(e) => {
                last_error = Some(e);
                sleep(Duration::from_millis(50)).await;
            }
        }
    }

    Err(last_error.unwrap())
}

pub async fn send_and_receive(message: &str) -> Result<String, std::io::Error> {
    let mut stream = connect_with_retry(10).await?;

    stream.write_all(message.as_bytes()).await?;
    stream.flush().await?;

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;

    let response = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(response)
}
