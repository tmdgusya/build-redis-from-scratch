pub struct Server;

impl Server {
    pub fn bind(_addr: &str) -> std::io::Result<Self> {
        todo!("Lesson 01: Server::bind() 구현 필요")
    }

    pub fn accept_one(&mut self) -> std::io::Result<Option<std::net::SocketAddr>> {
        todo!("Lesson 01: Server::accept_one() 구현 필요")
    }

    pub fn run_once(&mut self) -> std::io::Result<usize> {
        todo!("Lesson 02: Server::run_once() 구현 필요")
    }
}
