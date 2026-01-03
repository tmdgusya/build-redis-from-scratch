mod client;
mod server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tokio::spawn(async {
        if let Err(e) = server::run().await {
            eprintln!("Server error: {}", e);
        }
    });

    match client::send_and_receive("Hello, World!").await {
        Ok(response) => println!("Server responded: {}", response),
        Err(e) => eprintln!("Client error: {}", e),
    }

    Ok(())
}
