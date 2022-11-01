use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::fence;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use tokio::time::sleep;

#[derive(Debug)]
struct Mutex {
    flag: AtomicBool,
    value: u32
}

impl Mutex {
    fn new() -> Self {
        Self {
            flag: AtomicBool::new(false),
            value: 11,
        }
    }

    fn lock(&self) {
        // 获取锁
        while self.flag.
            compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err() 
        {
            thread::yield_now();
        }
        // 内存屏障， 同步unlock中的store操作，使之总是在屏障之后执行
        fence(Ordering::Acquire);
    }
    
    fn unlock(&self) {
        self.flag.store(false, Ordering::Release);
    }
}

// unsafe impl Sync for Mutex {}
// unsafe impl Send for Mutex {}


#[tokio::test]
async fn test_mutex() {
    let m = Mutex::new();
    let t = lock_test(&m, 1);
    let t1 = lock_test(&m,2);
    let t2 = lock_test(&m,3);
    tokio::join!(t1, t, t2);
}

async fn lock_test(
    lock: &Mutex, index: u64,
) {
    lock.lock();
    println!("test lock {index}");
    lock.unlock();
    sleep(Duration::from_secs(index)).await;
    println!("delay test worker {index}");
}