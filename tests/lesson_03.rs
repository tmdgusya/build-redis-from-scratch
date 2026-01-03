use build_redis::Server;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn find_available_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

fn run_server_iterations(server: &mut Server, n: usize) {
    for _ in 0..n {
        let _ = server.run_once();
    }
}

#[test]
fn test_multiple_clients_connect() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    let _client1 = TcpStream::connect(&addr).unwrap();
    let _client2 = TcpStream::connect(&addr).unwrap();
    let _client3 = TcpStream::connect(&addr).unwrap();
    std::thread::sleep(Duration::from_millis(50));

    run_server_iterations(&mut server, 10);
}

#[test]
fn test_multiple_clients_echo_independently() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    let mut client1 = TcpStream::connect(&addr).unwrap();
    let mut client2 = TcpStream::connect(&addr).unwrap();
    client1
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    client2
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    std::thread::sleep(Duration::from_millis(50));

    run_server_iterations(&mut server, 5);

    client1.write_all(b"from_client_1").unwrap();
    client2.write_all(b"from_client_2").unwrap();
    client1.flush().unwrap();
    client2.flush().unwrap();
    std::thread::sleep(Duration::from_millis(50));

    run_server_iterations(&mut server, 10);

    let mut buf1 = [0u8; 1024];
    let mut buf2 = [0u8; 1024];

    let n1 = client1.read(&mut buf1).unwrap();
    let n2 = client2.read(&mut buf2).unwrap();

    assert_eq!(
        &buf1[..n1],
        b"from_client_1",
        "Client 1 should receive its own echo"
    );
    assert_eq!(
        &buf2[..n2],
        b"from_client_2",
        "Client 2 should receive its own echo"
    );
}

#[test]
fn test_client_disconnect_doesnt_affect_others() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    let mut client1 = TcpStream::connect(&addr).unwrap();
    let client2 = TcpStream::connect(&addr).unwrap();
    let mut client3 = TcpStream::connect(&addr).unwrap();
    client1
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    client3
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    std::thread::sleep(Duration::from_millis(50));

    run_server_iterations(&mut server, 10);

    drop(client2);
    std::thread::sleep(Duration::from_millis(50));

    run_server_iterations(&mut server, 5);

    client1.write_all(b"still_alive_1").unwrap();
    client3.write_all(b"still_alive_3").unwrap();
    client1.flush().unwrap();
    client3.flush().unwrap();
    std::thread::sleep(Duration::from_millis(50));

    run_server_iterations(&mut server, 10);

    let mut buf1 = [0u8; 1024];
    let mut buf3 = [0u8; 1024];

    let n1 = client1.read(&mut buf1).unwrap();
    let n3 = client3.read(&mut buf3).unwrap();

    assert_eq!(&buf1[..n1], b"still_alive_1");
    assert_eq!(&buf3[..n3], b"still_alive_3");
}

#[test]
fn test_interleaved_messages() {
    let port = find_available_port();
    let addr = format!("127.0.0.1:{}", port);

    let mut server = Server::bind(&addr).unwrap();

    let mut client_a = TcpStream::connect(&addr).unwrap();
    let mut client_b = TcpStream::connect(&addr).unwrap();
    client_a
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    client_b
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    std::thread::sleep(Duration::from_millis(50));

    run_server_iterations(&mut server, 5);

    client_a.write_all(b"A1").unwrap();
    client_a.flush().unwrap();
    std::thread::sleep(Duration::from_millis(30));
    run_server_iterations(&mut server, 5);

    client_b.write_all(b"B1").unwrap();
    client_b.flush().unwrap();
    std::thread::sleep(Duration::from_millis(30));
    run_server_iterations(&mut server, 5);

    client_a.write_all(b"A2").unwrap();
    client_a.flush().unwrap();
    std::thread::sleep(Duration::from_millis(30));
    run_server_iterations(&mut server, 5);

    let mut buf = [0u8; 1024];

    let n = client_a.read(&mut buf).unwrap();
    assert_eq!(&buf[..n], b"A1");

    let n = client_b.read(&mut buf).unwrap();
    assert_eq!(&buf[..n], b"B1");

    let n = client_a.read(&mut buf).unwrap();
    assert_eq!(&buf[..n], b"A2");
}
