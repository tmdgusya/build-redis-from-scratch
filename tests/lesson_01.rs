use build_redis::Server;

use std::net::TcpStream;
use std::time::Duration;

fn find_available_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

#[test]
fn test_server_bind() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let server = Server::bind(&addr);
    assert!(server.is_ok(), "Server should bind successfully");
}

#[test]
fn test_server_bind_invalid_addr() {
    let server = Server::bind("invalid:addr:format");
    assert!(server.is_err(), "Server should fail on invalid address");
}

#[test]
fn test_accept_no_connection() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();
    let result = server.accept_one().unwrap();

    assert!(result.is_none(), "Should return None when no connection");
}

#[test]
fn test_accept_single_connection() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    let client = TcpStream::connect(&addr).unwrap();
    std::thread::sleep(Duration::from_millis(50));

    let result = server.accept_one().unwrap();
    assert!(result.is_some(), "Should accept a connection");

    let client_addr = result.unwrap();
    assert_eq!(client_addr.ip().to_string(), "127.0.0.1");

    drop(client);
}

#[test]
fn test_accept_multiple_connections() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    let _client1 = TcpStream::connect(&addr).unwrap();
    let _client2 = TcpStream::connect(&addr).unwrap();
    let _client3 = TcpStream::connect(&addr).unwrap();
    std::thread::sleep(Duration::from_millis(50));

    let mut accepted_count = 0;
    for _ in 0..5 {
        if let Ok(Some(_)) = server.accept_one() {
            accepted_count += 1;
        }
    }

    assert_eq!(accepted_count, 3, "Should accept exactly 3 connections");
}
