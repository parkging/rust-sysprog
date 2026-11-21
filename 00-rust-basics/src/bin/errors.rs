//! 00-3 `Result`와 `?`, 에러 처리
//!
//! C의 `-1` 반환과 `errno`를 Rust는 `Result<T, E>`로 표현한다.
//! `?`는 에러면 즉시 반환하고, 성공이면 값을 꺼낸다.
//! 확인: `strace -e trace=openat ./target/debug/errors`  (ENOENT가 그대로 보인다)

use std::fs::File;
use std::io::{self, Read};

fn read_first_byte(path: &str) -> io::Result<u8> {
    let mut f = File::open(path)?; // 실패하면 io::Error(errno)를 그대로 위로 전달
    let mut b = [0u8; 1];
    f.read_exact(&mut b)?;
    Ok(b[0])
}

fn main() {
    for path in ["/etc/hostname", "/no/such/file", "/root/.bashrc"] {
        match read_first_byte(path) {
            Ok(b) => println!("{path}: 첫 바이트 = {b}"),
            Err(e) => println!(
                "{path}: 실패 kind={:?} errno={:?} msg={e}",
                e.kind(),
                e.raw_os_error() // ENOENT=2, EACCES=13
            ),
        }
    }
}
