# 00. Rust 기초

Rust 문법을 한꺼번에 배우는 단계가 아니라, **이후 시스템 프로그래밍 예제를 읽기 위한 최소한**을 `syscall`과 연결해 익히는 단계입니다.

| 예제 | 개념 | 관련 syscall | 관찰 명령 |
|---|---|---|---|
| `hello` | cargo workspace, `println!` | `write` | `strace -e trace=write` |
| `ownership` | 소유권, 빌림, `Drop` | `openat`, `close` | `strace -e trace=openat,close` |
| `errors` | `Result`, `?`, errno | `openat` | `strace -e trace=openat` |
| `raw_syscall` | `unsafe`, `libc` FFI | `getpid`, `gettid`, `write` | `strace -e trace=getpid,gettid,write` |

## 실행

```bash
cargo run -p rust-basics --bin hello
strace -e trace=write ./target/debug/hello
```

## 실험 과제

1. `ownership.rs`에서 `consume` 호출 뒤 `f`를 다시 쓰는 줄의 주석을 풀고 컴파일 에러를 읽어 보세요.
2. `errors.rs`를 `strace`로 실행해서 `ENOENT`, `EACCES`가 실제 syscall 반환값에도 보이는지 확인하세요.
3. `raw_syscall.rs`에 스레드를 하나 만들고 `gettid`가 `getpid`와 다르게 나오는지 보세요. (10단계 예고)
