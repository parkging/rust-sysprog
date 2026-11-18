# rust-sysprog

리눅스 시스템 프로그래밍을 Rust 예제로 하나씩 익히면서 **리눅스 커널과 시스템 동작 원리**, **Rust 기본 개념**을 배우는 학습 리포입니다. 최종 목표는 커널 튜닝에 필요한 시스템 프로그래밍 역량입니다.

## 범위

- 기준 교재: 윤성우 『열혈 TCP/IP 소켓 프로그래밍』, 김선영 『Advanced! 리눅스 시스템 네트워크 프로그래밍 3판』 (리눅스만)
- **제외 (C 전용)**: 헤더와 묵시적 선언, feature test macro, 유닉스 역사와 표준안, 패딩/팩과 XDR, 정규 표현식과 C 문자열 함수, OpenMP, pthread API 자체(→ `std::thread`로 대체), `fd_set` 매크로
- **추가**: 페이지 캐시와 메모리 관리 종합, io_uring(파일과 네트워크), 튜닝 연결 실험
- 사용 크레이트: `nix`, `libc`, `io-uring` (필요할 때만 추가)

## 구조 (예정)

```
rust-sysprog/
├─ Cargo.toml          # workspace
├─ README.md           # 이 문서 (진도표)
├─ 00-rust-basics/     # 단계별 크레이트
│   ├─ README.md       # 개념, syscall, 커널 동작, 관찰 명령, 실험 과제
│   └─ src/bin/*.rs    # 예제 한 파일 = 개념 하나
└─ ...
```

크레이트 README에는 다음 네 가지를 적습니다: 불리는 syscall, 커널 안에서 일어나는 일, 직접 확인하는 명령(`strace`, `/proc`, `perf`), 실험 과제.

## 진도표 (목차)

체크박스는 예제를 올리고 설명까지 마치면 표시합니다. 예제 이름은 확정 전 가안입니다.

### 단계 한눈에 보기

| 단계 | 주제 | 참고 목차 | 핵심 커널 개념 | 예제 수 |
|---|---|---|---|---|
| 00 | Rust 기초 | - | syscall 경로 | 4 |
| 01 | 프로세스 | Ch01 | task_struct, COW | 5 |
| 02 | 파일 | Ch02 | VFS, 페이지 캐시, writeback | 10 |
| 03 | 메모리 | Ch04 | 가상 메모리, 페이지 폴트, OOM | 8 |
| 03.5 | 페이지 캐시와 메모리 관리 종합 | (신규) | LRU, dirty 튜닝, pagemap | 5 |
| 04 | mmap | Ch05 일부 | THP, 페이지 캐시 직접 매핑 | 7 |
| 05 | IPC | Ch05, Ch06 일부 | 커널 버퍼, futex, 세마포어 | 7 |
| 06 | 시그널 | Ch09 | 시그널 전달 | 5 |
| 07 | 소켓 기초 | Ch06 | 소켓 버퍼, 3-way handshake | 6 |
| 08 | 소켓 심화 | Ch06 | TCP 상태 머신, 혼잡제어 | 5 |
| 09 | I/O 멀티플렉싱 | Ch07 | 이벤트 큐, 레디 리스트 | 4 |
| 10 | 스레드 | Ch08 | futex, 캐시 라인 | 6 |
| 11 | 리얼타임과 리눅스 전용 기능 | Ch10, Ch11 | 스케줄러, 타이머 | 6 |
| 12 | 튜닝 연결 | (신규) | sysctl, cgroup v2 | 4 |
| 13 | io_uring | (신규) | SQ/CQ 링, 비동기 I/O | 8 |

### 00. Rust 기초
- [ ] 00-1 hello와 cargo workspace
- [ ] 00-2 소유권과 빌림
- [ ] 00-3 `Result`와 `?`, 에러 처리
- [ ] 00-4 `unsafe`와 `libc::getpid` 직접 호출

### 01. 프로세스
- [ ] 01-1 `fork` + `wait`
- [ ] 01-2 좀비 프로세스 만들기와 회수
- [ ] 01-3 `exec` 계열
- [ ] 01-4 `std::process::Command`
- [ ] 01-5 `posix_spawn`

### 02. 파일
- [ ] 02-1 fd 열기/읽기/쓰기, offset 공유(dup, fork)
- [ ] 02-2 `pread`/`pwrite`
- [ ] 02-3 버퍼드 vs 언버퍼드 I/O, syscall 횟수 비교
- [ ] 02-4 페이지 캐시 히트/미스 측정
- [ ] 02-5 `mincore`로 캐시 상주 페이지 확인
- [ ] 02-6 `fadvise`
- [ ] 02-7 readahead 관찰
- [ ] 02-8 `fsync`/`fdatasync`/`sync_file_range`와 dirty page
- [ ] 02-9 `O_DIRECT`와 `O_SYNC`
- [ ] 02-10 sparse 파일, `fallocate`, 대용량 파일, `sendfile`/`splice`/`copy_file_range`

### 03. 메모리
- [ ] 03-1 `/proc/self/smaps`, `status`, `maps` 파서
- [ ] 03-2 스택/힙/익명 매핑 레이아웃과 ASLR
- [ ] 03-3 demand paging과 페이지 폴트 카운트
- [ ] 03-4 할당기 동작 (`brk` vs `mmap`)
- [ ] 03-5 `mlock`/`mlockall`
- [ ] 03-6 COW 실험
- [ ] 03-7 메모리 압박 실험 (`vm.swappiness`, `vm.vfs_cache_pressure`)
- [ ] 03-8 OOM killer와 `oom_score_adj`

### 03.5. 페이지 캐시와 메모리 관리 종합
- [ ] 03.5-1 `/proc/meminfo`, `/proc/vmstat` 모니터 (Cached, Buffers, Active/Inactive, Dirty)
- [ ] 03.5-2 LRU와 active/inactive 리스트 관찰
- [ ] 03.5-3 dirty 비율 튜닝 실험 (`vm.dirty_*`)
- [ ] 03.5-4 `fincore`/`vmtouch` 직접 구현
- [ ] 03.5-5 `/proc/<pid>/pagemap`으로 가상 주소와 물리 프레임 확인

### 04. mmap
- [ ] 04-1 파일 mmap vs `read()`
- [ ] 04-2 `MAP_SHARED` vs `MAP_PRIVATE`
- [ ] 04-3 `msync`와 writeback
- [ ] 04-4 `madvise`
- [ ] 04-5 HugePage (THP와 hugetlb)
- [ ] 04-6 익명 공유 mmap (부모와 자식)
- [ ] 04-7 NUMA (`mbind`)

### 05. IPC
- [ ] 05-1 `pipe`
- [ ] 05-2 FIFO
- [ ] 05-3 POSIX 공유 메모리
- [ ] 05-4 POSIX 메시지 큐
- [ ] 05-5 POSIX 세마포어
- [ ] 05-6 SysV 세마포어
- [ ] 05-7 SysV 공유 메모리와 메시지 큐

### 06. 시그널
- [ ] 06-1 핸들러 등록과 async-signal-safe
- [ ] 06-2 시그널 블록 마스크
- [ ] 06-3 `SIGCHLD`와 자식 회수
- [ ] 06-4 `sigaltstack`
- [ ] 06-5 `signalfd`

### 07. 소켓 기초
- [ ] 07-1 TCP 에코 서버와 클라이언트
- [ ] 07-2 Iterative 서버와 TCP 이론 (handshake, 종료)
- [ ] 07-3 UDP 서버와 클라이언트, `connect`된 UDP
- [ ] 07-4 Unix 도메인 소켓
- [ ] 07-5 IPv6와 DNS 이름 변환
- [ ] 07-6 Half-close 우아한 종료

### 08. 소켓 심화
- [ ] 08-1 소켓 옵션 (`SO_REUSEADDR`, `TCP_NODELAY`, 버퍼 크기)
- [ ] 08-2 논블로킹 소켓
- [ ] 08-3 OOB 데이터
- [ ] 08-4 브로드캐스트
- [ ] 08-5 멀티캐스트

### 09. I/O 멀티플렉싱
- [ ] 09-1 `select`
- [ ] 09-2 `poll`
- [ ] 09-3 `epoll` 레벨 트리거와 엣지 트리거
- [ ] 09-4 epoll 에코 서버

### 10. 스레드
- [ ] 10-1 `std::thread`와 `Send`/`Sync`
- [ ] 10-2 `Mutex`/`Condvar`
- [ ] 10-3 atomic과 메모리 순서
- [ ] 10-4 false sharing
- [ ] 10-5 CPU affinity
- [ ] 10-6 스레드 풀 에코 서버

### 11. 리얼타임과 리눅스 전용 기능
- [ ] 11-1 시계 (`clock_gettime`)
- [ ] 11-2 `timerfd`
- [ ] 11-3 `eventfd`
- [ ] 11-4 리얼타임 시그널 (RTS)
- [ ] 11-5 AIO
- [ ] 11-6 `sched_setscheduler`(FIFO/RR)와 nice

### 12. 튜닝 연결
- [ ] 12-1 `/proc`, `/sys` 파서
- [ ] 12-2 sysctl 읽기/쓰기와 전후 비교 (`vm.*`)
- [ ] 12-3 네트워크 튜닝 실험 (`net.*`, 소켓 버퍼)
- [ ] 12-4 cgroup v2로 CPU와 메모리 제한

### 13. io_uring
- [ ] 13-1 NOP 제출과 완료, SQ/CQ 링 구조
- [ ] 13-2 파일 읽기/쓰기
- [ ] 13-3 여러 파일 동시 읽기, 배치 제출
- [ ] 13-4 등록 버퍼와 등록 fd
- [ ] 13-5 TCP 에코 서버 (`Accept`/`Recv`/`Send`)
- [ ] 13-6 멀티샷과 provided buffer ring
- [ ] 13-7 `SQPOLL`과 `IOPOLL`
- [ ] 13-8 벤치마크: `read`/AIO/epoll/io_uring 비교

## 진행 순서

`00 → 01 → 02 → 03 → 03.5 → 04`로 파일과 메모리를 먼저 다지고, 이후 튜닝 우선순위에 따라 `09`, `10`, `13` 등을 앞당길 수 있습니다. `13`(io_uring)은 `09`(epoll)와 `11`(AIO) 이후에 하면 비교하기 좋습니다.

## 진행 방식

1. 예제 1~2개를 올립니다.
2. 코드와 "이 syscall에서 커널이 무엇을 하는가"를 설명합니다.
3. 직접 실행하고 `strace`, `/proc`, `perf`로 확인합니다.
4. 실험 과제를 풀고, 완료하면 체크박스를 표시합니다.

## 준비물

- Rust stable (`rustup`, `cargo`, `clippy`, `rustfmt`)
- `strace`, `perf`, `ltrace`
- io_uring은 커널 5.1 이상, 멀티샷과 buffer ring은 6.0 이상이 필요하며, 컨테이너나 seccomp 환경에서는 막혀 있을 수 있습니다.
