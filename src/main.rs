use build_redis::Server;

fn main() -> std::io::Result<()> {
    let mut server = Server::bind("127.0.0.1:6379")?;
    println!("Server listening on 127.0.0.1:6379");

    loop {
        if let Some(addr) = server.accept_one()? {
            println!("New connection from {}", addr);
        }
    }
}
