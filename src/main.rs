mod client;
mod server;

fn main() -> Result<(), std::io::Error> {
    let handle = std::thread::spawn(|| {
        if let Err(e) = server::run() {
            eprintln!("Server error: {}", e);
        }
    });

    match client::send_and_receive("Hello, World!") {
        Ok(response) => println!("Server responded: {}", response),
        Err(e) => eprintln!("Client error: {}", e),
    }

    handle.join().unwrap();
    Ok(())
}
