# Lesson 02: Event Loop 기초

## 학습 목표

- 무한 event loop 구현
- 클라이언트 연결에서 데이터 읽기/쓰기
- 단일 클라이언트 Echo 서버 완성

## 핵심 개념

### 1. Event Loop 구조

```rust
loop {
    // 1. 이벤트 대기
    poll.poll(&mut events, None)?;
    
    // 2. 이벤트 처리
    for event in events.iter() {
        match event.token() {
            SERVER => { /* accept */ }
            client => { /* read/write */ }
        }
    }
}
```

### 2. 클라이언트 등록

새 연결을 accept한 후, 해당 소켓도 Poll에 등록해야 합니다.

```rust
let (mut stream, addr) = listener.accept()?;
let token = Token(next_token);
next_token += 1;

poll.registry().register(
    &mut stream,
    token,
    Interest::READABLE | Interest::WRITABLE
)?;
```

### 3. Non-blocking I/O 처리

mio의 소켓은 non-blocking입니다. `WouldBlock` 에러를 적절히 처리해야 합니다.

```rust
match stream.read(&mut buf) {
    Ok(0) => { /* 연결 종료 */ }
    Ok(n) => { /* n 바이트 읽음 */ }
    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        // 아직 데이터 없음, 다음 이벤트 대기
    }
    Err(e) => { /* 실제 에러 */ }
}
```

### 4. Echo 서버 동작

```
Client          Server
  |               |
  |--- "hello" -->|
  |               | (read)
  |<-- "hello" ---|
  |               | (write)
```

## 과제

`src/lib.rs`에 다음을 추가 구현하세요:

### 추가할 메서드

```rust
impl Server {
    /// Event loop를 한 번 실행 (poll + 이벤트 처리)
    /// 처리된 이벤트 수 반환
    pub fn run_once(&mut self) -> std::io::Result<usize>;
}
```

### 동작 요구사항

1. `run_once()` 호출 시:
   - 새 연결이 있으면 accept
   - 기존 연결에서 데이터가 있으면 읽고 그대로 echo
   - 연결이 종료되면 정리

2. 단일 클라이언트 처리 (이번 레슨에서는 1개 연결만)

## 힌트

1. `Server` 구조체에 추가할 필드:
   - 현재 연결된 클라이언트 `Option<TcpStream>`
   - 클라이언트의 Token

2. `Interest::READABLE | Interest::WRITABLE`로 등록

3. Echo: `stream.read()` → `stream.write_all()`

## 테스트 실행

```bash
cargo test --test lesson_02
```
