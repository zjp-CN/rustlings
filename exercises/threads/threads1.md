# 方法一：`AtomicU32`

`dbg_time` 用于打印时间信息，可以移除掉。

```rust
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use std::sync::atomic::{AtomicU32, Ordering};

struct JobStatus {
    jobs_completed: AtomicU32,
}
fn main() {
    let status = Arc::new(JobStatus { jobs_completed: AtomicU32::new(0), });
    let status_shared = status.clone();
    let now = Instant::now();

    macro_rules! dbg_time {
        ($i:ident) => {{
            let $i = now.elapsed().as_millis();
            dbg!($i);
        }};
    }

    thread::spawn(move || {
        dbg_time!(before_for);
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.jobs_completed.fetch_add(1, Ordering::SeqCst);
            dbg_time!(in_for);
        }
        dbg_time!(after_for);
    });
    while status.jobs_completed.load(Ordering::SeqCst) < 10 {
        println!("waiting... ");
        dbg_time!(load); // 这里打印的内容可能位于 `println` 之后（不知道为什么）
        thread::sleep(Duration::from_millis(500));
        dbg_time!(sleep);
    }
}
```

# 方法二：`lock`

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct JobStatus {
    jobs_completed: u32,
}
fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    let now = Instant::now();

    macro_rules! dbg_time {
        ($i:ident) => {{
            let $i = now.elapsed().as_millis();
            dbg!($i);
        }};
    }

    thread::spawn(move || {
        dbg_time!(before_for);
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
            dbg_time!(in_for);
        }
        dbg_time!(after_for);
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        dbg_time!(load);
        thread::sleep(Duration::from_millis(500));
        dbg_time!(sleep);
    }
}
```

# 打印结果

```text
waiting...
[exercises/threads/threads1.rs:41] load = 0
[exercises/threads/threads1.rs:31] before_for = 0
[exercises/threads/threads1.rs:35] in_for = 251
[exercises/threads/threads1.rs:43] sleep = 500
[exercises/threads/threads1.rs:41] load = 501
waiting...
[exercises/threads/threads1.rs:35] in_for = 502
[exercises/threads/threads1.rs:35] in_for = 753
[exercises/threads/threads1.rs:43] sleep = 1001
[exercises/threads/threads1.rs:41] load = 1001
waiting...
[exercises/threads/threads1.rs:35] in_for = 1004
[exercises/threads/threads1.rs:35] in_for = 1254
[exercises/threads/threads1.rs:43] sleep = 1501
[exercises/threads/threads1.rs:41] load = 1501
waiting...
[exercises/threads/threads1.rs:35] in_for = 1504
[exercises/threads/threads1.rs:35] in_for = 1754
[exercises/threads/threads1.rs:43] sleep = 2001
[exercises/threads/threads1.rs:41] load = 2001
waiting...
[exercises/threads/threads1.rs:35] in_for = 2004
[exercises/threads/threads1.rs:35] in_for = 2254
[exercises/threads/threads1.rs:43] sleep = 2501
[exercises/threads/threads1.rs:41] load = 2501
waiting...
[exercises/threads/threads1.rs:35] in_for = 2504
[exercises/threads/threads1.rs:37] after_for = 2504
[exercises/threads/threads1.rs:43] sleep = 3001
```
