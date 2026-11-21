//! 00-2 소유권과 빌림
//!
//! 소유권은 "누가 이 자원을 해제할 책임이 있는가"를 컴파일 시점에 정한다.
//! 시스템 프로그래밍에서는 자원이 곧 fd, 메모리 매핑, 락이다.
//! 소유자가 스코프를 벗어나면 `Drop`이 실행되어 `close(fd)`가 호출된다.
//! 확인: `strace -e trace=openat,close ./target/debug/ownership`

use std::fs::File;
use std::io::Read;

/// 소유권을 가져간다. 함수가 끝나면 `file`이 drop되어 fd가 닫힌다.
fn consume(mut file: File) -> usize {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.len()
} // <- 여기서 close(fd)

/// 빌린다. 소유권은 호출자에게 남아 있다.
fn borrow(file: &mut File) -> usize {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.len()
}

fn main() {
    let mut f = File::open("/etc/hostname").unwrap(); // openat
    println!("빌려서 읽기: {} bytes", borrow(&mut f));
    // 두 번째 읽기가 0 bytes인 이유: 파일 offset은 Rust 값이 아니라 커널의
    // open file description(`struct file`)에 있다. 첫 읽기에서 EOF까지 이동했으므로
    // 같은 fd로 다시 읽으면 더 읽을 것이 없다. (02-1에서 dup/fork와 함께 다시 본다)
    // f는 아직 유효하다. 소유권을 넘기면 이후에는 쓸 수 없다.
    println!("소유권 넘겨서 읽기: {} bytes", consume(f));
    // println!("{:?}", f); // 컴파일 에러: f는 이미 move 됐다 (use after close 방지)
    println!("끝");
}
