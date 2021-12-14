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
