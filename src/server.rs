use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;

    if n == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Client disconnected",
        ));
    }

    let message = String::from_utf8_lossy(&buffer[..n]);
    println!("client says: {}", message);

    stream.write_all(&buffer[..n]).await?;
    stream.flush().await?;
    Ok(())
}

pub async fn run() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Server listening on 127.0.0.1:6379 (tokio)");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("handle_client error: {}", e);
            }
        });
    }
}
