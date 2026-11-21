//! 00-4 `unsafe`와 `libc` 직접 호출
//!
//! std가 감싸 주는 syscall을 `libc` 크레이트로 직접 부른다.
//! 컴파일러가 안전성을 보장할 수 없는 호출(FFI, raw pointer)은 `unsafe` 안에서만 가능하다.
//! 핵심 습관: unsafe 블록은 작게 만들고, 바깥에는 안전한 함수로 감싼다.
//! 확인: `strace -e trace=getpid,write,gettid ./target/debug/raw_syscall`

/// `getpid`는 인자가 없고 실패하지 않아서 안전한 래퍼로 감쌀 수 있다.
fn getpid() -> i32 {
    // SAFETY: 인자가 없고 부작용이 없는 syscall이다.
    unsafe { libc::getpid() }
}

/// `libc::syscall`로 syscall 번호를 직접 지정해서 호출한다. (glibc 래퍼 우회)
fn gettid_raw() -> i64 {
    // SAFETY: SYS_gettid는 인자가 없고 항상 성공한다.
    unsafe { libc::syscall(libc::SYS_gettid) }
}

/// fd 1에 직접 쓴다. 포인터와 길이를 우리가 맞춰야 해서 unsafe다.
fn write_stdout(msg: &str) -> isize {
    // SAFETY: msg는 유효한 슬라이스이고 길이는 as_bytes().len()이다.
    unsafe { libc::write(1, msg.as_ptr().cast(), msg.len()) }
}

fn main() {
    println!("pid = {}", getpid());
    println!("tid = {} (단일 스레드면 pid와 같다)", gettid_raw());
    let n = write_stdout("raw write 호출\n");
    println!("write 반환값 = {n}");
}
