// Rust 所有权规则教学代码
// 主题：所有权、移动、克隆、复制

/// 示例 1: 所有权基本规则
/// Rust 的三条所有权规则
pub fn ownership_rules() {
    println!("\n=== 示例 1: 所有权基本规则 ===");
    
    // 规则 1: Rust 中的每一个值都有一个所有者
    let s1 = String::from("hello");
    println!("s1 的所有者是当前作用域");
    
    // 规则 2: 值在任一时刻有且只有一个所有者
    // let s2 = s1;  // s1 的所有权移动到 s2
    // println!("{}", s1);  // 错误！s1 已经失效
    
    // 规则 3: 当所有者离开作用域，这个值将被丢弃
    {
        let s3 = String::from("world");
        println!("s3 = {}", s3);
    }  // s3 在这里离开作用域并被丢弃
    
    // println!("{}", s3);  // 错误！s3 已经不存在
    
    println!("所有权规则确保内存安全");
}

/// 示例 2: 变量与数据的交互 - 移动
/// String 类型的移动语义
pub fn move_semantics() {
    println!("\n=== 示例 2: 变量与数据的交互 - 移动 ===");
    
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    
    // s1 的所有权移动到 s2
    let s2 = s1;
    println!("s2 = {}", s2);
    
    // println!("{}", s1);  // 错误！s1 已经失效
    
    println!("移动后，s1 不再有效，避免了双重释放");
}

/// 示例 3: 变量与数据的交互 - 克隆
/// 使用 clone 进行深拷贝
pub fn clone_semantics() {
    println!("\n=== 示例 3: 变量与数据的交互 - 克隆 ===");
    
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 深拷贝
    
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("克隆会复制堆上的数据，开销较大");
}

/// 示例 4: 栈上数据的复制
/// 实现了 Copy trait 的类型
pub fn copy_semantics() {
    println!("\n=== 示例 4: 栈上数据的复制 ===");
    
    // 整数类型实现了 Copy trait
    let x = 5;
    let y = x;  // 复制，不是移动
    
    println!("x = {}, y = {}", x, y);
    println!("x 仍然有效，因为整数实现了 Copy trait");
    
    // 其他实现了 Copy 的类型
    let a = true;
    let b = a;
    println!("布尔值: a = {}, b = {}", a, b);
    
    let c = 'A';
    let d = c;
    println!("字符: c = {}, d = {}", c, d);
    
    let e = (1, 2, 3);
    let f = e;
    println!("元组: e = {:?}, f = {:?}", e, f);
}

/// 示例 5: 所有权与函数
/// 将值传递给函数会移动或复制
pub fn ownership_and_functions() {
    println!("\n=== 示例 5: 所有权与函数 ===");
    
    let s = String::from("hello");
    takes_ownership(s);  // s 的所有权移动到函数里
    // println!("{}", s);  // 错误！s 已经失效
    
    let x = 5;
    makes_copy(x);  // x 是 Copy 的，所以可以继续使用
    println!("x 仍然有效: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("函数接收: {}", some_string);
}  // some_string 在这里离开作用域并被丢弃

fn makes_copy(some_integer: i32) {
    println!("函数接收: {}", some_integer);
}  // some_integer 离开作用域，但不会有特殊操作

/// 示例 6: 返回值与所有权
/// 返回值可以转移所有权
pub fn return_values_and_ownership() {
    println!("\n=== 示例 6: 返回值与所有权 ===");
    
    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2);  // 错误！s2 已经失效
    println!("s3 = {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // 返回值移动给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回值移动给调用者
}

/// 示例 7: 返回多个值
/// 使用元组返回多个值
pub fn return_multiple_values() {
    println!("\n=== 示例 7: 返回多个值 ===");
    
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    
    println!("字符串 '{}' 的长度是 {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // 返回字符串和长度
}

/// 示例 8: 所有权转移的时机
/// 理解所有权何时转移
pub fn ownership_transfer_timing() {
    println!("\n=== 示例 8: 所有权转移的时机 ===");
    
    let s = String::from("hello");
    
    // 赋值时转移
    let s1 = s;
    
    // 传参时转移
    let s2 = String::from("world");
    let s3 = transfer(s2);

    // 返回时转移
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn transfer(s: String) -> String {
    s
}

/// 示例 9: 部分移动
/// 结构体字段的部分移动
pub fn partial_move() {
    println!("\n=== 示例 9: 部分移动 ===");
    
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let name = person.name;  // name 字段被移动
    let age = person.age;    // age 被复制（u32 实现了 Copy）
    
    println!("姓名: {}, 年龄: {}", name, age);
    // println!("{}", person.name);  // 错误！name 已被移动
    println!("年龄仍可访问: {}", person.age);  // age 可以访问
}

/// 示例 10: 所有权与作用域
/// 作用域结束时释放资源
pub fn ownership_and_scope() {
    println!("\n=== 示例 10: 所有权与作用域 ===");
    
    let outer = String::from("外部");
    
    {
        let inner = String::from("内部");
        println!("内部作用域: {}, {}", outer, inner);
    }  // inner 在这里被丢弃
    
    println!("外部作用域: {}", outer);
    // println!("{}", inner);  // 错误！inner 已不存在
}

/// 示例 11: 所有权与 Vec
/// Vec 的所有权行为
pub fn ownership_with_vec() {
    println!("\n=== 示例 11: 所有权与 Vec ===");
    
    let v1 = vec![1, 2, 3];
    let v2 = v1;  // v1 的所有权移动到 v2
    
    // println!("{:?}", v1);  // 错误！v1 已失效
    println!("v2 = {:?}", v2);
    
    // 克隆 Vec
    let v3 = vec![4, 5, 6];
    let v4 = v3.clone();
    println!("v3 = {:?}, v4 = {:?}", v3, v4);
}

/// 示例 12: 所有权与 Box
/// Box 智能指针的所有权
pub fn ownership_with_box() {
    println!("\n=== 示例 12: 所有权与 Box ===");
    
    let b1 = Box::new(5);
    println!("b1 = {}", b1);
    
    let b2 = b1;  // 所有权移动
    // println!("{}", b1);  // 错误！b1 已失效
    println!("b2 = {}", b2);
    
    // Box 指向的数据在堆上
    let b3 = Box::new(String::from("hello"));
    println!("b3 = {}", b3);
}

/// 示例 13: 实际应用 - 字符串拼接
pub fn practical_string_concatenation() {
    println!("\n=== 示例 13: 实际应用 - 字符串拼接 ===");
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    
    // + 运算符会获取 s1 的所有权
    let s3 = s1 + &s2;
    // println!("{}", s1);  // 错误！s1 已被移动
    println!("s2 = {}", s2);  // s2 仍然有效
    println!("s3 = {}", s3);
    
    // 使用 format! 宏不会获取所有权
    let s4 = String::from("Hello, ");
    let s5 = String::from("Rust!");
    let s6 = format!("{}{}", s4, s5);
    println!("s4 = {}, s5 = {}, s6 = {}", s4, s5, s6);
}

/// 示例 14: 实际应用 - 交换值
pub fn practical_swap_values() {
    println!("\n=== 示例 14: 实际应用 - 交换值 ===");
    
    let mut s1 = String::from("first");
    let mut s2 = String::from("second");
    
    println!("交换前: s1 = {}, s2 = {}", s1, s2);
    
    // 使用 std::mem::swap 交换值
    std::mem::swap(&mut s1, &mut s2);
    
    println!("交换后: s1 = {}, s2 = {}", s1, s2);
}

/// 示例 15: 实际应用 - 构建器模式
pub fn practical_builder_pattern() {
    println!("\n=== 示例 15: 实际应用 - 构建器模式 ===");
    
    struct Config {
        name: String,
        value: i32,
    }
    
    impl Config {
        fn new() -> Self {
            Config {
                name: String::new(),
                value: 0,
            }
        }
        
        fn name(mut self, name: String) -> Self {
            self.name = name;
            self  // 返回 self 的所有权
        }
        
        fn value(mut self, value: i32) -> Self {
            self.value = value;
            self
        }
        
        fn build(self) -> Self {
            self
        }
    }
    
    let config = Config::new()
        .name(String::from("app"))
        .value(42)
        .build();
    
    println!("配置: name = {}, value = {}", config.name, config.value);
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 所有权规则教学代码             ║");
    println!("╚════════════════════════════════════════╝");
    
    ownership_rules();
    move_semantics();
    clone_semantics();
    copy_semantics();
    ownership_and_functions();
    return_values_and_ownership();
    return_multiple_values();
    ownership_transfer_timing();
    partial_move();
    ownership_and_scope();
    ownership_with_vec();
    ownership_with_box();
    practical_string_concatenation();
    practical_swap_values();
    practical_builder_pattern();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  所有权是 Rust 最独特的特性！       ║");
    println!("║  它保证了内存安全而无需垃圾回收     ║");
    println!("╚════════════════════════════════════════╝");
}

