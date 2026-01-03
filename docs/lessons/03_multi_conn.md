# Lesson 03: 다중 연결 처리

## 학습 목표

- 여러 클라이언트 동시 처리
- `HashMap`으로 연결 관리
- Token 기반 클라이언트 식별

## 핵심 개념

### 1. 연결 관리 자료구조

```rust
use std::collections::HashMap;
use mio::net::TcpStream;
use mio::Token;

struct Server {
    // ...
    connections: HashMap<Token, TcpStream>,
    next_token: usize,
}
```

### 2. Token 할당 전략

```
Token(0) = Listener (고정)
Token(1) = 첫 번째 클라이언트
Token(2) = 두 번째 클라이언트
...
```

```rust
const LISTENER: Token = Token(0);

fn next_token(&mut self) -> Token {
    let token = Token(self.next_token);
    self.next_token += 1;
    token
}
```

### 3. 이벤트 처리 분기

```rust
for event in events.iter() {
    match event.token() {
        LISTENER => {
            // accept 후 HashMap에 추가
            let token = self.next_token();
            self.connections.insert(token, stream);
        }
        token => {
            // HashMap에서 찾아서 처리
            if let Some(stream) = self.connections.get_mut(&token) {
                // read/write
            }
        }
    }
}
```

### 4. 연결 종료 처리

```rust
// read()가 0 반환 시 연결 종료
if n == 0 {
    poll.registry().deregister(&mut stream)?;
    self.connections.remove(&token);
}
```

## 과제

`src/lib.rs`를 수정하여 다중 연결을 지원하세요.

### 수정 사항

1. `Server` 구조체에 `HashMap<Token, TcpStream>` 추가
2. `run_once()`가 여러 클라이언트를 동시에 처리
3. 각 클라이언트별로 독립적인 echo 동작

### 동작 요구사항

```
Client A: "hello" --> Server --> "hello" (A에게)
Client B: "world" --> Server --> "world" (B에게)
```

## 힌트

1. Token 0은 listener 전용
2. `HashMap::get_mut()`으로 스트림 접근
3. 연결 종료 시 `deregister()` + `remove()` 필수

## 테스트 실행

```bash
cargo test --test lesson_03
```
