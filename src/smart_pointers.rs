/// Rust 智能指针与包装类型教学代码
///
/// 智能指针是一种数据结构，它不仅像指针一样存储内存地址，
/// 还拥有额外的元数据和功能。

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::borrow::Cow;
use std::ops::Deref;

/// 示例 1: Box<T> - 堆分配
///
/// Box 是最简单的智能指针，用于在堆上分配数据
pub fn box_basics() {
    println!("\n=== 示例 1: Box<T> - 堆分配 ===");

    // 在堆上分配一个整数
    let b = Box::new(5);
    println!("堆上的值: {}", b);

    // Box 实现了 Deref trait，可以像引用一样使用
    let x = *b + 10;
    println!("解引用后: {}", x);

    // 在堆上分配一个大数组
    let large_array = Box::new([0; 1000]);
    println!("大数组的长度: {}", large_array.len());

    // Box 离开作用域时会自动释放堆内存
    println!("\nBox 的用途:");
    println!("  - 在堆上分配数据");
    println!("  - 避免栈溢出");
    println!("  - 转移大量数据的所有权时避免复制");
}

/// 示例 2: Box<T> 与递归类型
///
/// Box 可以用于定义递归数据结构
pub fn box_recursive_types() {
    println!("\n=== 示例 2: Box<T> 与递归类型 ===");

    // 链表节点
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>), // 递归类型必须使用 Box
        Nil,
    }

    use List::{Cons, Nil};

    // 创建链表: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表: {:?}", list);

    // 二叉树节点
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    }

    // 创建二叉树
    let tree = TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: None,
        })),
    };
    println!("二叉树: {:?}", tree);

    println!("\nBox 解决递归类型问题:");
    println!("  - 编译器需要知道类型的大小");
    println!("  - Box 的大小是固定的（指针大小）");
    println!("  - 可以创建无限嵌套的数据结构");
}

/// 示例 3: Rc<T> - 引用计数
///
/// Rc 允许多个所有者共享同一数据
pub fn rc_basics() {
    println!("\n=== 示例 3: Rc<T> - 引用计数 ===");

    // 创建一个 Rc
    let a = Rc::new(String::from("hello"));
    println!("a 的引用计数: {}", Rc::strong_count(&a));

    // 克隆 Rc（增加引用计数）
    let b = Rc::clone(&a);
    println!("克隆后 a 的引用计数: {}", Rc::strong_count(&a));
    println!("b 的引用计数: {}", Rc::strong_count(&b));

    {
        let c = Rc::clone(&a);
        println!("再次克隆后的引用计数: {}", Rc::strong_count(&a));
        println!("c 的值: {}", c);
    } // c 离开作用域，引用计数减 1

    println!("c 离开作用域后的引用计数: {}", Rc::strong_count(&a));

    println!("\nRc 的特点:");
    println!("  - 允许多个所有者");
    println!("  - 只能用于单线程");
    println!("  - 数据是不可变的");
    println!("  - 引用计数为 0 时自动释放");
}

/// 示例 4: Rc<T> 共享数据
///
/// 使用 Rc 在多个数据结构间共享数据
pub fn rc_shared_data() {
    println!("\n=== 示例 4: Rc<T> 共享数据 ===");

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<Node>>,
    }

    // 创建共享节点
    let shared = Rc::new(Node {
        value: 3,
        next: None,
    });

    // 两个节点都指向同一个共享节点
    let node1 = Node {
        value: 1,
        next: Some(Rc::clone(&shared)),
    };

    let node2 = Node {
        value: 2,
        next: Some(Rc::clone(&shared)),
    };

    println!("node1: {:?}", node1);
    println!("node2: {:?}", node2);
    println!("共享节点的引用计数: {}", Rc::strong_count(&shared));

    println!("\n共享数据的优势:");
    println!("  - 避免数据复制");
    println!("  - 多个结构可以访问同一数据");
    println!("  - 自动内存管理");
}

/// 示例 5: RefCell<T> - 内部可变性
///
/// RefCell 允许在不可变引用的情况下修改数据
pub fn refcell_basics() {
    println!("\n=== 示例 5: RefCell<T> - 内部可变性 ===");

    let data = RefCell::new(5);
    println!("初始值: {}", data.borrow());

    // 可变借用并修改
    *data.borrow_mut() += 10;
    println!("修改后: {}", data.borrow());

    // 多个不可变借用
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("r1: {}, r2: {}", r1, r2);
    } // 借用在这里结束

    // 可变借用
    *data.borrow_mut() *= 2;
    println!("再次修改后: {}", data.borrow());

    println!("\nRefCell 的特点:");
    println!("  - 运行时检查借用规则");
    println!("  - 允许内部可变性");
    println!("  - 违反借用规则会 panic");
    println!("  - 只能用于单线程");
}

/// 示例 6: Rc<RefCell<T>> - 共享可变数据
///
/// 结合 Rc 和 RefCell 实现多个所有者的可变数据
pub fn rc_refcell_combination() {
    println!("\n=== 示例 6: Rc<RefCell<T>> - 共享可变数据 ===");

    #[derive(Debug)]
    struct SharedData {
        value: i32,
    }

    // 创建共享的可变数据
    let data = Rc::new(RefCell::new(SharedData { value: 0 }));

    // 创建多个所有者
    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);

    // 通过 data1 修改
    data1.borrow_mut().value += 10;
    println!("通过 data1 修改后: {:?}", data.borrow());

    // 通过 data2 修改
    data2.borrow_mut().value += 20;
    println!("通过 data2 修改后: {:?}", data.borrow());

    // 所有引用都能看到最新的值
    println!("data 的值: {:?}", data.borrow());
    println!("引用计数: {}", Rc::strong_count(&data));

    println!("\nRc<RefCell<T>> 的用途:");
    println!("  - 多个所有者需要修改数据");
    println!("  - 图、树等复杂数据结构");
    println!("  - 只能用于单线程");
}

/// 示例 7: Cell<T> - 简单的内部可变性
///
/// Cell 提供了更简单但受限的内部可变性
pub fn cell_basics() {
    println!("\n=== 示例 7: Cell<T> - 简单的内部可变性 ===");

    let c = Cell::new(5);
    println!("初始值: {}", c.get());

    // 直接设置新值
    c.set(10);
    println!("设置后: {}", c.get());

    // 替换并返回旧值
    let old = c.replace(20);
    println!("旧值: {}, 新值: {}", old, c.get());

    // Cell 用于 Copy 类型
    let x = Cell::new(100);
    let y = Cell::new(200);
    
    // 交换两个 Cell 的值
    x.swap(&y);
    println!("交换后 x: {}, y: {}", x.get(), y.get());

    println!("\nCell vs RefCell:");
    println!("  Cell:");
    println!("    - 只能用于 Copy 类型");
    println!("    - 通过复制获取和设置值");
    println!("    - 没有运行时开销");
    println!("  RefCell:");
    println!("    - 可以用于任何类型");
    println!("    - 通过借用获取引用");
    println!("    - 有运行时借用检查开销");
}

/// 示例 8: Arc<T> - 原子引用计数
///
/// Arc 是线程安全的 Rc
pub fn arc_basics() {
    println!("\n=== 示例 8: Arc<T> - 原子引用计数 ===");

    use std::thread;

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("原始数据: {:?}", data);
    println!("引用计数: {}", Arc::strong_count(&data));

    let mut handles = vec![];

    // 创建多个线程，每个线程都持有数据的引用
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {} 看到的数据: {:?}", i, data_clone);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有线程完成后的引用计数: {}", Arc::strong_count(&data));

    println!("\nArc 的特点:");
    println!("  - 线程安全的引用计数");
    println!("  - 可以在多线程间共享数据");
    println!("  - 数据是不可变的");
    println!("  - 比 Rc 有额外的原子操作开销");
}

/// 示例 9: Arc<Mutex<T>> - 线程安全的共享可变数据
///
/// 结合 Arc 和 Mutex 实现多线程的共享可变数据
pub fn arc_mutex_combination() {
    println!("\n=== 示例 9: Arc<Mutex<T>> - 线程安全的共享可变数据 ===");

    use std::thread;

    // 创建共享的可变数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 创建 10 个线程，每个线程都增加计数器
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取锁并修改数据
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("线程 {} 增加计数器", i);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());

    println!("\nArc<Mutex<T>> 的特点:");
    println!("  - 多线程共享可变数据");
    println!("  - Mutex 提供互斥访问");
    println!("  - lock() 会阻塞直到获取锁");
    println!("  - 锁在离开作用域时自动释放");
}

/// 示例 10: Cow<T> - 写时克隆
///
/// Cow 可以延迟克隆，只在需要修改时才克隆
pub fn cow_basics() {
    println!("\n=== 示例 10: Cow<T> - 写时克隆 ===");

    // 从借用创建 Cow
    let s = String::from("hello");
    let cow1: Cow<str> = Cow::Borrowed(&s);
    println!("借用的 Cow: {}", cow1);

    // 从拥有的数据创建 Cow
    let cow2: Cow<str> = Cow::Owned(String::from("world"));
    println!("拥有的 Cow: {}", cow2);

    // 只读访问不会克隆
    let cow3: Cow<str> = Cow::Borrowed("rust");
    println!("只读访问: {}", cow3);

    // 修改时才会克隆
    let mut cow4: Cow<str> = Cow::Borrowed("hello");
    println!("修改前: {}", cow4);
    cow4.to_mut().push_str(" world"); // 这里会克隆
    println!("修改后: {}", cow4);

    println!("\nCow 的优势:");
    println!("  - 避免不必要的克隆");
    println!("  - 只在需要修改时才克隆");
    println!("  - 适合读多写少的场景");
}

/// 示例 11: Cow<T> 实际应用
///
/// 使用 Cow 优化字符串处理
pub fn cow_practical() {
    println!("\n=== 示例 11: Cow<T> 实际应用 ===");

    // 函数可能修改也可能不修改字符串
    fn process_text(text: &str) -> Cow<str> {
        if text.contains("bad") {
            // 需要修改，返回拥有的数据
            Cow::Owned(text.replace("bad", "good"))
        } else {
            // 不需要修改，返回借用
            Cow::Borrowed(text)
        }
    }

    let text1 = "This is a good text";
    let result1 = process_text(text1);
    println!("文本 1: {} (借用: {})", result1, matches!(result1, Cow::Borrowed(_)));

    let text2 = "This is a bad text";
    let result2 = process_text(text2);
    println!("文本 2: {} (借用: {})", result2, matches!(result2, Cow::Borrowed(_)));

    println!("\n实际应用场景:");
    println!("  - 配置文件处理");
    println!("  - 文本替换和过滤");
    println!("  - 路径规范化");
}

/// 示例 12: 自定义智能指针 - 基础
///
/// 实现一个简单的智能指针
pub fn custom_smart_pointer_basics() {
    println!("\n=== 示例 12: 自定义智能指针 - 基础 ===");

    // 自定义智能指针
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 实现 Deref trait，使其可以像引用一样使用
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // 实现 Drop trait，自定义清理逻辑
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("清理 MyBox");
        }
    }

    let x = MyBox::new(5);
    println!("MyBox 的值: {}", *x); // 使用 Deref

    let y = MyBox::new(String::from("hello"));
    println!("字符串长度: {}", y.len()); // Deref 强制转换

    println!("\n自定义智能指针需要:");
    println!("  - Deref trait: 解引用行为");
    println!("  - Drop trait: 清理逻辑");
}

/// 示例 13: 自定义智能指针 - 引用计数
///
/// 实现一个简化的引用计数智能指针
pub fn custom_smart_pointer_rc() {
    println!("\n=== 示例 13: 自定义智能指针 - 引用计数 ===");

    use std::cell::RefCell;

    // 简化的引用计数智能指针
    struct SimpleRc<T> {
        data: *const T,
        ref_count: *const RefCell<usize>,
    }

    impl<T> SimpleRc<T> {
        fn new(value: T) -> Self {
            let boxed = Box::new(value);
            let data = Box::into_raw(boxed);
            let ref_count = Box::into_raw(Box::new(RefCell::new(1)));

            SimpleRc { data, ref_count }
        }

        fn count(&self) -> usize {
            unsafe { (*self.ref_count).borrow().clone() }
        }
    }

    impl<T> Clone for SimpleRc<T> {
        fn clone(&self) -> Self {
            unsafe {
                *(*self.ref_count).borrow_mut() += 1;
            }
            SimpleRc {
                data: self.data,
                ref_count: self.ref_count,
            }
        }
    }

    impl<T> Deref for SimpleRc<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.data }
        }
    }

    impl<T> Drop for SimpleRc<T> {
        fn drop(&mut self) {
            unsafe {
                *(*self.ref_count).borrow_mut() -= 1;
                if *(*self.ref_count).borrow() == 0 {
                    // 释放数据和引用计数
                    drop(Box::from_raw(self.data as *mut T));
                    drop(Box::from_raw(self.ref_count as *mut RefCell<usize>));
                    println!("释放 SimpleRc 的数据");
                }
            }
        }
    }

    let rc1 = SimpleRc::new(String::from("hello"));
    println!("rc1 引用计数: {}", rc1.count());

    {
        let rc2 = rc1.clone();
        println!("rc2 引用计数: {}", rc2.count());
        println!("rc1 引用计数: {}", rc1.count());
    }

    println!("rc2 离开作用域后 rc1 引用计数: {}", rc1.count());

    println!("\n实现引用计数需要:");
    println!("  - 共享的引用计数器");
    println!("  - Clone trait 增加计数");
    println!("  - Drop trait 减少计数并清理");
}

/// 示例 14: 实际应用 - 图数据结构
///
/// 使用智能指针实现图
pub fn practical_graph() {
    println!("\n=== 示例 14: 实际应用 - 图数据结构 ===");

    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node {
        value: i32,
        neighbors: RefCell<Vec<Rc<Node>>>,
    }

    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                neighbors: RefCell::new(Vec::new()),
            })
        }

        fn add_neighbor(&self, neighbor: Rc<Node>) {
            self.neighbors.borrow_mut().push(neighbor);
        }
    }

    // 创建节点
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);

    // 建立连接
    node1.add_neighbor(Rc::clone(&node2));
    node1.add_neighbor(Rc::clone(&node3));
    node2.add_neighbor(Rc::clone(&node3));

    println!("节点 1 的值: {}", node1.value);
    println!("节点 1 的邻居数: {}", node1.neighbors.borrow().len());
    println!("节点 2 的引用计数: {}", Rc::strong_count(&node2));

    println!("\n图数据结构需要:");
    println!("  - Rc: 多个节点可以指向同一个节点");
    println!("  - RefCell: 可以修改邻居列表");
}

/// 示例 15: 实际应用 - 缓存系统
///
/// 使用智能指针实现简单的缓存
pub fn practical_cache() {
    println!("\n=== 示例 15: 实际应用 - 缓存系统 ===");

    use std::collections::HashMap;

    struct Cache {
        data: Arc<Mutex<HashMap<String, String>>>,
    }

    impl Cache {
        fn new() -> Self {
            Cache {
                data: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn get(&self, key: &str) -> Option<String> {
            self.data.lock().unwrap().get(key).cloned()
        }

        fn set(&self, key: String, value: String) {
            self.data.lock().unwrap().insert(key, value);
        }

        fn clone_handle(&self) -> Self {
            Cache {
                data: Arc::clone(&self.data),
            }
        }
    }

    let cache = Cache::new();
    cache.set("name".to_string(), "Rust".to_string());
    cache.set("version".to_string(), "1.70".to_string());

    println!("缓存中的 name: {:?}", cache.get("name"));
    println!("缓存中的 version: {:?}", cache.get("version"));

    // 克隆缓存句柄
    let cache2 = cache.clone_handle();
    println!("通过 cache2 获取 name: {:?}", cache2.get("name"));

    println!("\n缓存系统的特点:");
    println!("  - Arc<Mutex<T>>: 线程安全的共享可变数据");
    println!("  - 可以在多个地方访问同一缓存");
    println!("  - 适合多线程环境");
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Rust 智能指针与包装类型教学代码      ║");
    println!("╚════════════════════════════════════════╝");

    box_basics();
    box_recursive_types();
    rc_basics();
    rc_shared_data();
    refcell_basics();
    rc_refcell_combination();
    cell_basics();
    arc_basics();
    arc_mutex_combination();
    cow_basics();
    cow_practical();
    custom_smart_pointer_basics();
    custom_smart_pointer_rc();
    practical_graph();
    practical_cache();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  智能指针是 Rust 的强大工具！        ║");
    println!("╚════════════════════════════════════════╝\n");
}

