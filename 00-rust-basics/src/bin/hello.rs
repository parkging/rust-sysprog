//! 00-1 hello와 cargo workspace
//!
//! `println!`이 끝까지 내려가면 `write(1, ...)` syscall 하나가 된다.
//! 확인: `strace -e trace=write ./target/debug/hello`

fn main() {
    println!("hello, rust-sysprog");
}
