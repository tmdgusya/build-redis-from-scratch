use std::io::{Read, Write};
use std::net::TcpStream;

fn connect() -> Result<TcpStream, std::io::Error> {
    TcpStream::connect("127.0.0.1:6379")
}

pub fn send_and_receive(message: &str) -> Result<String, std::io::Error> {
    let mut stream = connect()?;

    stream.write_all(message.as_bytes())?;
    stream.flush()?;

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer)?;

    let response = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(response)
}
