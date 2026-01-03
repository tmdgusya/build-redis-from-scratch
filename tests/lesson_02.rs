use build_redis::Server;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn find_available_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

#[test]
fn test_run_once_no_events() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();
    let count = server.run_once().unwrap();

    assert_eq!(count, 0, "Should return 0 when no events");
}

#[test]
fn test_run_once_accepts_connection() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();
    let _client = TcpStream::connect(&addr).unwrap();
    std::thread::sleep(Duration::from_millis(50));

    let count = server.run_once().unwrap();
    assert!(count >= 1, "Should process at least 1 event (accept)");
}

#[test]
fn test_echo_single_message() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    let mut client = TcpStream::connect(&addr).unwrap();
    client
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    std::thread::sleep(Duration::from_millis(50));

    for _ in 0..5 {
        server.run_once().unwrap();
    }

    client.write_all(b"hello").unwrap();
    client.flush().unwrap();
    std::thread::sleep(Duration::from_millis(50));

    for _ in 0..5 {
        server.run_once().unwrap();
    }

    let mut buf = [0u8; 1024];
    let n = client.read(&mut buf).unwrap();

    assert_eq!(&buf[..n], b"hello", "Server should echo back the message");
}

#[test]
fn test_echo_multiple_messages() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    let mut client = TcpStream::connect(&addr).unwrap();
    client
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    std::thread::sleep(Duration::from_millis(50));

    let messages = ["first", "second", "third"];

    for msg in &messages {
        for _ in 0..3 {
            server.run_once().unwrap();
        }

        client.write_all(msg.as_bytes()).unwrap();
        client.flush().unwrap();
        std::thread::sleep(Duration::from_millis(50));

        for _ in 0..3 {
            server.run_once().unwrap();
        }

        let mut buf = [0u8; 1024];
        let n = client.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], msg.as_bytes(), "Should echo: {}", msg);
    }
}

#[test]
fn test_client_disconnect() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    {
        let client = TcpStream::connect(&addr).unwrap();
        std::thread::sleep(Duration::from_millis(50));

        for _ in 0..3 {
            server.run_once().unwrap();
        }

        drop(client);
    }

    std::thread::sleep(Duration::from_millis(50));

    let result = server.run_once();
    assert!(
        result.is_ok(),
        "Server should handle client disconnect gracefully"
    );
}
