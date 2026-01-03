use std::{
    io::{BufReader, BufWriter, Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn do_something(stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buffer = [0u8; 1024];
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream.try_clone().unwrap());

    match reader.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Client disconnected",
                ));
            }
            let message = String::from_utf8_lossy(&buffer[..n]);
            println!("client says: {}", message);
            writer.write_all(&buffer[..n])
        }
        Err(e) => {
            println!("read() error: {}", e);
            Err(e)
        }
    }?;

    writer.flush()?;
    Ok(())
}

pub fn run() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    if let Err(e) = do_something(stream) {
                        println!("do_something() error: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("accept() error: {}", e);
            }
        }
    }
    Ok(())
}
