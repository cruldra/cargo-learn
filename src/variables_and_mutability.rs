// Rust 变量与可变性教学代码
// 主题：let、mut 关键字的使用

/// 示例 1: 不可变变量（默认行为）
/// Rust 中的变量默认是不可变的（immutable）
pub fn immutable_variables() {
    println!("\n=== 示例 1: 不可变变量 ===");
    
    let x = 5;
    println!("x 的值是: {}", x);
    
    // 下面这行代码会导致编译错误，因为 x 是不可变的
    // x = 6;
    // 错误信息: cannot assign twice to immutable variable `x`
    
    println!("不可变变量一旦绑定值后，就不能再改变");
}

/// 示例 2: 可变变量（使用 mut 关键字）
/// 使用 mut 关键字可以让变量变为可变的
pub fn mutable_variables() {
    println!("\n=== 示例 2: 可变变量 ===");
    
    let mut y = 5;
    println!("y 的初始值是: {}", y);
    
    y = 6;
    println!("y 的新值是: {}", y);
    
    y = y + 10;
    println!("y 再次改变后的值是: {}", y);
}

/// 示例 3: 变量遮蔽（Shadowing）
/// 可以使用相同的变量名重新声明变量
pub fn variable_shadowing() {
    println!("\n=== 示例 3: 变量遮蔽 ===");
    
    let x = 5;
    println!("第一次声明 x = {}", x);
    
    let x = x + 1;
    println!("第二次声明 x = {}", x);
    
    {
        let x = x * 2;
        println!("内部作用域中 x = {}", x);
    }
    
    println!("外部作用域中 x = {}", x);
}

/// 示例 4: 遮蔽 vs 可变性
/// 遮蔽允许改变变量的类型，而 mut 不允许
pub fn shadowing_vs_mutability() {
    println!("\n=== 示例 4: 遮蔽 vs 可变性 ===");
    
    // 使用遮蔽可以改变类型
    let spaces = "   ";
    println!("spaces 是字符串: '{}'", spaces);
    
    let spaces = spaces.len();
    println!("spaces 现在是数字: {}", spaces);
    
    // 使用 mut 不能改变类型
    let mut count = "123";
    println!("count 是字符串: '{}'", count);
    
    count = "456";
    println!("count 仍然是字符串: '{}'", count);
    
    // 下面这行会导致编译错误
    // count = count.len();
    // 错误: expected `&str`, found `usize`
}

/// 示例 5: 常量（const）
/// 常量与不可变变量的区别
pub fn constants_example() {
    println!("\n=== 示例 5: 常量 ===");
    
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);
    
    // 常量的特点：
    // 1. 必须使用 const 关键字
    // 2. 必须标注类型
    // 3. 只能设置为常量表达式，不能是运行时计算的值
    // 4. 可以在任何作用域中声明，包括全局作用域
    // 5. 命名约定：全大写字母，单词间用下划线分隔
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("三小时的秒数: {}", THREE_HOURS_IN_SECONDS);
}

/// 示例 6: 未使用的变量
/// 使用下划线前缀避免未使用变量的警告
pub fn unused_variables() {
    println!("\n=== 示例 6: 未使用的变量 ===");
    
    // 这会产生警告
    // let unused = 42;
    
    // 使用下划线前缀避免警告
    let _unused = 42;
    println!("使用下划线前缀可以避免未使用变量的警告");
    
    // 或者只使用下划线
    let _ = "这个值会被立即丢弃";
}

/// 示例 7: 解构赋值
/// 使用 let 进行模式匹配和解构
pub fn destructuring() {
    println!("\n=== 示例 7: 解构赋值 ===");
    
    let (x, y) = (1, 2);
    println!("x = {}, y = {}", x, y);
    
    let (a, mut b) = (3, 4);
    println!("a = {}, b = {}", a, b);
    
    b = 5;
    println!("修改后 b = {}", b);
    
    // 解构数组
    let arr = [10, 20, 30];
    let [first, second, third] = arr;
    println!("数组元素: {}, {}, {}", first, second, third);
}

/// 示例 8: 可变引用
/// 理解可变性在引用中的应用
pub fn mutable_references() {
    println!("\n=== 示例 8: 可变引用 ===");
    
    let mut s = String::from("hello");
    println!("原始字符串: {}", s);
    
    // 创建可变引用
    let r = &mut s;
    r.push_str(", world");
    println!("修改后的字符串: {}", r);
    
    // 注意：在同一作用域中，只能有一个可变引用
    println!("最终字符串: {}", s);
}

/// 示例 9: 类型推断与显式类型标注
pub fn type_inference() {
    println!("\n=== 示例 9: 类型推断与显式类型标注 ===");
    
    // 类型推断
    let x = 42;
    println!("推断类型 x = {}", x);
    
    // 显式类型标注
    let y: i32 = 42;
    println!("显式类型 y = {}", y);
    
    // 可变变量的类型标注
    let mut z: f64 = 3.14;
    println!("可变浮点数 z = {}", z);
    z = 2.71;
    println!("修改后 z = {}", z);
}

/// 示例 10: 延迟初始化
/// 变量可以先声明后初始化
pub fn delayed_initialization() {
    println!("\n=== 示例 10: 延迟初始化 ===");
    
    let x: i32;
    
    // 在使用前必须初始化
    x = 42;
    println!("延迟初始化的 x = {}", x);
    
    // 条件初始化
    let y: i32;
    let condition = true;
    
    if condition {
        y = 100;
    } else {
        y = 200;
    }
    
    println!("条件初始化的 y = {}", y);
}

/// 示例 11: 作用域与生命周期
pub fn scope_and_lifetime() {
    println!("\n=== 示例 11: 作用域与生命周期 ===");
    
    let outer = 1;
    println!("外部作用域 outer = {}", outer);
    
    {
        let inner = 2;
        println!("内部作用域 inner = {}", inner);
        println!("内部作用域可以访问 outer = {}", outer);
    }
    
    // 下面这行会导致编译错误，因为 inner 已经超出作用域
    // println!("outer 作用域无法访问 inner = {}", inner);
    
    println!("inner 变量已经被销毁");
}

/// 示例 12: 实际应用场景
pub fn practical_examples() {
    println!("\n=== 示例 12: 实际应用场景 ===");
    
    // 场景 1: 累加器
    let mut sum = 0;
    for i in 1..=5 {
        sum += i;
    }
    println!("1 到 5 的和: {}", sum);
    
    // 场景 2: 字符串构建
    let mut message = String::from("Rust");
    message.push_str(" 是");
    message.push_str("一门");
    message.push_str("系统编程语言");
    println!("{}", message);
    
    // 场景 3: 计数器
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
    }
    println!("计数器值: {}", counter);
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 变量与可变性教学代码           ║");
    println!("╚════════════════════════════════════════╝");
    
    immutable_variables();
    mutable_variables();
    variable_shadowing();
    shadowing_vs_mutability();
    constants_example();
    unused_variables();
    destructuring();
    mutable_references();
    type_inference();
    delayed_initialization();
    scope_and_lifetime();
    practical_examples();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  所有示例运行完毕！                  ║");
    println!("╚════════════════════════════════════════╝");
}

