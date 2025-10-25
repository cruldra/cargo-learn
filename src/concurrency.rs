/// Rust 并发编程教学代码
///
/// 并发编程允许程序同时执行多个任务
/// Rust 的类型系统保证了并发安全

use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::Duration;

/// 示例 1: 线程创建与管理
///
/// 使用 thread::spawn 创建新线程
pub fn thread_basics() {
    println!("\n=== 示例 1: 线程创建与管理 ===");

    // 创建一个新线程
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程继续执行
    for i in 1..=3 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    // 等待子线程完成
    handle.join().unwrap();

    println!("\n线程基础:");
    println!("  - thread::spawn 创建新线程");
    println!("  - join() 等待线程完成");
    println!("  - 线程并发执行");
}

/// 示例 2: 线程返回值
///
/// 线程可以返回值
pub fn thread_return_value() {
    println!("\n=== 示例 2: 线程返回值 ===");

    let handle = thread::spawn(|| {
        println!("计算中...");
        thread::sleep(Duration::from_millis(500));
        42 // 返回值
    });

    println!("等待计算结果...");
    let result = handle.join().unwrap();
    println!("计算结果: {}", result);

    // 多个线程并行计算
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let result = i * i;
                println!("线程 {} 计算: {} * {} = {}", i, i, i, result);
                result
            })
        })
        .collect();

    let results: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    println!("所有结果: {:?}", results);
    println!("总和: {}", results.iter().sum::<i32>());

    println!("\n线程返回值:");
    println!("  - join() 返回 Result<T>");
    println!("  - 可以收集多个线程的结果");
}

/// 示例 3: move 闭包
///
/// 使用 move 将所有权转移到线程
pub fn thread_move_closure() {
    println!("\n=== 示例 3: move 闭包 ===");

    let data = vec![1, 2, 3, 4, 5];

    // 必须使用 move 将 data 的所有权转移到线程
    let handle = thread::spawn(move || {
        println!("线程中的数据: {:?}", data);
        data.iter().sum::<i32>()
    });

    // data 已经被移动，这里不能再使用
    // println!("{:?}", data); // 错误！

    let sum = handle.join().unwrap();
    println!("数据总和: {}", sum);

    println!("\nmove 闭包:");
    println!("  - move 关键字转移所有权");
    println!("  - 避免悬垂引用");
    println!("  - 保证线程安全");
}

/// 示例 4: 消息传递 - 单生产者单消费者
///
/// 使用 channel 在线程间传递消息
pub fn message_passing_basic() {
    println!("\n=== 示例 4: 消息传递 - 单生产者单消费者 ===");

    // 创建一个通道
    let (tx, rx) = mpsc::channel();

    // 生产者线程
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for msg in messages {
            println!("发送: {}", msg);
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 消费者（主线程）
    for received in rx {
        println!("接收: {}", received);
    }

    println!("\n消息传递:");
    println!("  - mpsc::channel() 创建通道");
    println!("  - tx.send() 发送消息");
    println!("  - rx.recv() 接收消息");
    println!("  - 通道关闭后迭代结束");
}

/// 示例 5: 消息传递 - 多生产者单消费者
///
/// 多个线程向同一个通道发送消息
pub fn message_passing_multiple_producers() {
    println!("\n=== 示例 5: 消息传递 - 多生产者单消费者 ===");

    let (tx, rx) = mpsc::channel();

    // 创建多个生产者
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for j in 0..3 {
                let msg = format!("生产者 {} 的消息 {}", i, j);
                println!("发送: {}", msg);
                tx_clone.send(msg).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    // 丢弃原始发送者，否则接收者会一直等待
    drop(tx);

    // 接收所有消息
    for received in rx {
        println!("接收: {}", received);
    }

    println!("\n多生产者:");
    println!("  - tx.clone() 克隆发送者");
    println!("  - mpsc = multiple producer, single consumer");
    println!("  - drop(tx) 关闭通道");
}

/// 示例 6: 共享状态 - Mutex
///
/// 使用 Mutex 保护共享数据
pub fn shared_state_mutex() {
    println!("\n=== 示例 6: 共享状态 - Mutex ===");

    // Arc = Atomic Reference Counting（原子引用计数）
    // Mutex = Mutual Exclusion（互斥锁）
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取锁
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("线程 {} 增加计数器: {}", i, *num);
            // 锁在这里自动释放
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());

    println!("\nMutex:");
    println!("  - Mutex::new() 创建互斥锁");
    println!("  - lock() 获取锁");
    println!("  - 锁在作用域结束时自动释放");
    println!("  - Arc 允许多个所有者");
}

/// 示例 7: 共享状态 - RwLock
///
/// 读写锁允许多个读者或一个写者
pub fn shared_state_rwlock() {
    println!("\n=== 示例 7: 共享状态 - RwLock ===");

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // 创建多个读者线程
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_guard = data_clone.read().unwrap();
            println!("读者 {} 读取: {:?}", i, *read_guard);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    // 创建一个写者线程
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut write_guard = data_clone.write().unwrap();
        write_guard.push(4);
        println!("写者添加元素: {:?}", *write_guard);
    });
    handles.push(handle);

    // 再创建一个读者
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let read_guard = data_clone.read().unwrap();
        println!("读者 3 读取: {:?}", *read_guard);
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nRwLock:");
    println!("  - read() 获取读锁（可多个）");
    println!("  - write() 获取写锁（独占）");
    println!("  - 适合读多写少的场景");
}

/// 示例 8: Send 和 Sync trait
///
/// Send: 可以在线程间转移所有权
/// Sync: 可以在线程间共享引用
pub fn send_and_sync_traits() {
    println!("\n=== 示例 8: Send 和 Sync trait ===");

    // i32 实现了 Send 和 Sync
    let num = 42;
    let handle = thread::spawn(move || {
        println!("i32 是 Send: {}", num);
    });
    handle.join().unwrap();

    // Arc<T> 实现了 Send 和 Sync（如果 T: Send + Sync）
    let shared_data = Arc::new(vec![1, 2, 3]);
    let shared_clone = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        println!("Arc<Vec<i32>> 是 Send + Sync: {:?}", shared_clone);
    });
    handle.join().unwrap();

    // Rc<T> 不是 Send，不能跨线程使用
    // let rc_data = Rc::new(vec![1, 2, 3]);
    // let handle = thread::spawn(move || {
    //     println!("{:?}", rc_data); // 错误！Rc 不是 Send
    // });

    println!("\nSend 和 Sync:");
    println!("  - Send: 可以转移到其他线程");
    println!("  - Sync: 可以被多个线程引用");
    println!("  - 大多数类型都实现了 Send 和 Sync");
    println!("  - Rc、RefCell 等不是 Send");
    println!("  - Arc、Mutex 是 Send + Sync");
}

/// 示例 9: 原子类型
///
/// 原子操作不需要锁
pub fn atomic_types() {
    println!("\n=== 示例 9: 原子类型 ===");

    use std::sync::atomic::{AtomicUsize, Ordering};

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // 原子操作，无需锁
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
            println!("线程 {} 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", counter.load(Ordering::SeqCst));

    println!("\n原子类型:");
    println!("  - AtomicBool, AtomicI32, AtomicUsize 等");
    println!("  - fetch_add, fetch_sub, swap, compare_exchange");
    println!("  - 无锁并发，性能更高");
    println!("  - 适合简单的计数器和标志");
}

/// 示例 10: 内存顺序（Ordering）
///
/// 原子操作的内存顺序
pub fn memory_ordering() {
    println!("\n=== 示例 10: 内存顺序（Ordering） ===");

    use std::sync::atomic::{AtomicBool, Ordering};

    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        println!("设置标志为 true");
        flag_clone.store(true, Ordering::Release);
    });

    // 等待标志变为 true
    while !flag.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(10));
    }

    println!("检测到标志为 true");
    handle.join().unwrap();

    println!("\n内存顺序:");
    println!("  - Relaxed: 最宽松，无同步保证");
    println!("  - Acquire: 读操作，防止后续操作重排到前面");
    println!("  - Release: 写操作，防止前面操作重排到后面");
    println!("  - AcqRel: 读写操作，结合 Acquire 和 Release");
    println!("  - SeqCst: 顺序一致性，最严格（默认推荐）");
}

/// 示例 11: 屏障（Barrier）
///
/// 让多个线程在某个点同步
pub fn barrier_synchronization() {
    println!("\n=== 示例 11: 屏障（Barrier） ===");

    use std::sync::Barrier;

    let barrier = Arc::new(Barrier::new(5));
    let mut handles = vec![];

    for i in 0..5 {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("线程 {} 准备中...", i);
            thread::sleep(Duration::from_millis(i as u64 * 100));

            println!("线程 {} 到达屏障", i);
            barrier_clone.wait();

            println!("线程 {} 继续执行", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n屏障:");
    println!("  - Barrier::new(n) 创建屏障");
    println!("  - wait() 等待所有线程到达");
    println!("  - 所有线程同时继续执行");
}

/// 示例 12: 条件变量（Condvar）
///
/// 线程间的条件等待和通知
pub fn condition_variable() {
    println!("\n=== 示例 12: 条件变量（Condvar） ===");

    use std::sync::Condvar;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    // 等待线程
    let handle = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();

        println!("等待线程: 等待条件...");
        while !*started {
            started = cvar.wait(started).unwrap();
        }

        println!("等待线程: 条件满足，继续执行");
    });

    // 主线程
    thread::sleep(Duration::from_millis(500));

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    *started = true;
    println!("主线程: 设置条件并通知");
    cvar.notify_one();
    drop(started);

    handle.join().unwrap();

    println!("\n条件变量:");
    println!("  - Condvar::new() 创建条件变量");
    println!("  - wait() 等待条件");
    println!("  - notify_one() 通知一个线程");
    println!("  - notify_all() 通知所有线程");
}

/// 示例 13: 线程局部存储
///
/// 每个线程有自己的变量副本
pub fn thread_local_storage() {
    println!("\n=== 示例 13: 线程局部存储 ===");

    use std::cell::RefCell;

    thread_local! {
        static COUNTER: RefCell<u32> = RefCell::new(0);
    }

    let mut handles = vec![];

    for i in 0..3 {
        let handle = thread::spawn(move || {
            COUNTER.with(|c| {
                for _ in 0..3 {
                    *c.borrow_mut() += 1;
                    println!("线程 {}: 计数 = {}", i, c.borrow());
                    thread::sleep(Duration::from_millis(100));
                }
            });
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n线程局部存储:");
    println!("  - thread_local! 宏定义");
    println!("  - 每个线程独立的变量");
    println!("  - 避免同步开销");
}

/// 示例 14: 作用域线程（Scoped Threads）
///
/// 线程可以借用栈上的数据
pub fn scoped_threads() {
    println!("\n=== 示例 14: 作用域线程（Scoped Threads） ===");

    let mut data = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        // 可以借用 data，不需要 move
        s.spawn(|| {
            println!("线程 1 读取: {:?}", data);
        });

        s.spawn(|| {
            println!("线程 2 读取: {:?}", data);
        });

        // 所有作用域线程在这里自动 join
    });

    // 作用域结束后，可以继续使用 data
    data.push(6);
    println!("主线程修改后: {:?}", data);

    println!("\n作用域线程:");
    println!("  - thread::scope() 创建作用域");
    println!("  - 可以借用栈上的数据");
    println!("  - 自动等待所有线程完成");
    println!("  - 避免 Arc 和 move");
}

/// 示例 15: 简单的线程池
///
/// 使用通道实现简单的线程池
pub fn simple_thread_pool() {
    println!("\n=== 示例 15: 简单的线程池 ===");

    type Job = Box<dyn FnOnce() + Send + 'static>;

    struct ThreadPool {
        workers: Vec<Option<thread::JoinHandle<()>>>,
        sender: Option<mpsc::Sender<Job>>,
    }

    impl ThreadPool {
        fn new(size: usize) -> ThreadPool {
            let (sender, receiver) = mpsc::channel::<Job>();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                let receiver = Arc::clone(&receiver);
                let worker = thread::spawn(move || loop {
                    let job = receiver.lock().unwrap().recv();
                    match job {
                        Ok(job) => {
                            println!("工作线程 {} 执行任务", id);
                            job();
                        }
                        Err(_) => {
                            println!("工作线程 {} 关闭", id);
                            break;
                        }
                    }
                });
                workers.push(Some(worker));
            }

            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }

        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.take());
            for worker in &mut self.workers {
                if let Some(thread) = worker.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    // 使用线程池
    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("任务 {} 开始", i);
            thread::sleep(Duration::from_millis(200));
            println!("任务 {} 完成", i);
        });
    }

    // pool 在这里被 drop，等待所有任务完成
    drop(pool);
    thread::sleep(Duration::from_millis(100));

    println!("\n线程池:");
    println!("  - 复用线程，避免创建开销");
    println!("  - 限制并发数量");
    println!("  - 任务队列管理");
}

/// 示例 16: 实际应用 - 并行计算
///
/// 使用多线程加速计算
pub fn practical_parallel_computation() {
    println!("\n=== 示例 16: 实际应用 - 并行计算 ===");

    // 计算一个范围内所有数字的平方和
    fn sum_of_squares(start: u64, end: u64) -> u64 {
        (start..end).map(|x| x * x).sum()
    }

    let total = 1_000_000u64;
    let num_threads = 4;
    let chunk_size = total / num_threads as u64;

    // 单线程版本
    let start = std::time::Instant::now();
    let single_result = sum_of_squares(0, total);
    let single_duration = start.elapsed();
    println!("单线程结果: {}, 耗时: {:?}", single_result, single_duration);

    // 多线程版本
    let start = std::time::Instant::now();
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i as u64 * chunk_size;
        let end = if i == num_threads - 1 {
            total
        } else {
            (i as u64 + 1) * chunk_size
        };

        let handle = thread::spawn(move || sum_of_squares(start, end));
        handles.push(handle);
    }

    let multi_result: u64 = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .sum();
    let multi_duration = start.elapsed();

    println!("多线程结果: {}, 耗时: {:?}", multi_result, multi_duration);
    println!("加速比: {:.2}x", single_duration.as_secs_f64() / multi_duration.as_secs_f64());

    println!("\n并行计算:");
    println!("  - 将任务分割成多个块");
    println!("  - 每个线程处理一个块");
    println!("  - 合并结果");
    println!("  - 适合 CPU 密集型任务");
}

/// 示例 17: 实际应用 - 生产者消费者模式
///
/// 经典的并发模式
pub fn practical_producer_consumer() {
    println!("\n=== 示例 17: 实际应用 - 生产者消费者模式 ===");

    let (tx, rx) = mpsc::channel();
    let buffer_size = Arc::new(Mutex::new(0));

    // 生产者
    let buffer_clone = Arc::clone(&buffer_size);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            let item = format!("商品-{}", i);
            println!("生产: {}", item);
            tx.send(item).unwrap();

            let mut size = buffer_clone.lock().unwrap();
            *size += 1;
            println!("  缓冲区大小: {}", *size);

            thread::sleep(Duration::from_millis(100));
        }
    });

    // 消费者
    let buffer_clone = Arc::clone(&buffer_size);
    let consumer = thread::spawn(move || {
        for item in rx {
            thread::sleep(Duration::from_millis(150));
            println!("消费: {}", item);

            let mut size = buffer_clone.lock().unwrap();
            *size -= 1;
            println!("  缓冲区大小: {}", *size);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    println!("\n生产者消费者:");
    println!("  - 解耦生产和消费");
    println!("  - 缓冲区平衡速度差异");
    println!("  - 通道天然支持此模式");
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Rust 并发编程教学代码                ║");
    println!("╚════════════════════════════════════════╝");

    thread_basics();
    thread_return_value();
    thread_move_closure();
    message_passing_basic();
    message_passing_multiple_producers();
    shared_state_mutex();
    shared_state_rwlock();
    send_and_sync_traits();
    atomic_types();
    memory_ordering();
    barrier_synchronization();
    condition_variable();
    thread_local_storage();
    scoped_threads();
    simple_thread_pool();
    practical_parallel_computation();
    practical_producer_consumer();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  并发是 Rust 的核心优势之一！        ║");
    println!("╚════════════════════════════════════════╝\n");
}

