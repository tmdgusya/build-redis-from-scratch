# Lesson 01: TCP Listener 기본

## 학습 목표

- `mio::net::TcpListener`로 TCP 서버 생성
- `Poll`과 `Token` 개념 이해
- 단일 연결 accept 처리

## 핵심 개념

### 1. mio의 구성 요소

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│    Poll     │────▶│   Events    │────▶│   Token     │
│ (epoll 래퍼) │     │ (이벤트 버퍼) │     │ (fd 식별자)  │
└─────────────┘     └─────────────┘     └─────────────┘
```

### 2. Token이란?

Token은 어떤 소켓에서 이벤트가 발생했는지 식별하는 정수입니다.

```rust
use mio::Token;

const SERVER: Token = Token(0);  // listener용
// Token(1), Token(2), ... 는 클라이언트용
```

### 3. 기본 흐름

```rust
// 1. Poll 생성
let mut poll = Poll::new()?;

// 2. TcpListener 생성 및 등록
let mut listener = TcpListener::bind(addr)?;
poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;

// 3. 이벤트 대기
let mut events = Events::with_capacity(128);
poll.poll(&mut events, None)?;  // blocking

// 4. 이벤트 처리
for event in events.iter() {
    if event.token() == SERVER {
        let (connection, addr) = listener.accept()?;
        println!("New connection from {}", addr);
    }
}
```

### 4. Interest (관심 이벤트)

```rust
Interest::READABLE  // 읽기 가능할 때 알림 (accept, read)
Interest::WRITABLE  // 쓰기 가능할 때 알림 (write)
Interest::READABLE | Interest::WRITABLE  // 둘 다
```

## 과제

`src/lib.rs`에 다음을 구현하세요:

### 구현할 구조체/함수

```rust
pub struct Server {
    // TODO: 필요한 필드
}

impl Server {
    /// 지정된 주소에 바인딩된 서버 생성
    pub fn bind(addr: &str) -> std::io::Result<Self>;
    
    /// 연결 하나를 accept하고 주소 반환
    /// 연결이 없으면 None 반환
    pub fn accept_one(&mut self) -> std::io::Result<Option<std::net::SocketAddr>>;
}
```

## 힌트

1. `Server` 구조체에 필요한 것:
   - `Poll`
   - `TcpListener` 
   - `Events`

2. `accept_one()`에서:
   - `poll.poll()`로 이벤트 대기 (짧은 timeout)
   - `listener.accept()` 호출
   - `WouldBlock` 에러는 `Ok(None)`으로 처리

## 테스트 실행

```bash
cargo test --test lesson_01
```

## 참고 자료

- [mio 공식 문서](https://docs.rs/mio/latest/mio/)
- [mio TcpListener 예제](https://docs.rs/mio/latest/mio/net/struct.TcpListener.html)
