// Rust 函数定义与调用教学代码
// 主题：函数声明、参数、返回值、表达式

/// 示例 1: 基本函数定义与调用
/// 使用 fn 关键字定义函数
pub fn basic_function() {
    println!("\n=== 示例 1: 基本函数定义与调用 ===");
    
    // 调用无参数无返回值的函数
    greet();
    say_hello();
    
    // 函数定义
    fn greet() {
        println!("你好，Rust！");
    }
    
    fn say_hello() {
        println!("Hello, World!");
    }
    
    println!("函数使用 fn 关键字定义");
}

/// 示例 2: 带参数的函数
/// 函数参数必须声明类型
pub fn function_with_parameters() {
    println!("\n=== 示例 2: 带参数的函数 ===");
    
    print_number(42);
    print_sum(10, 20);
    greet_person("张三", 25);
    
    fn print_number(x: i32) {
        println!("数字是: {}", x);
    }
    
    fn print_sum(a: i32, b: i32) {
        println!("{} + {} = {}", a, b, a + b);
    }
    
    fn greet_person(name: &str, age: u32) {
        println!("你好，{}！你今年 {} 岁。", name, age);
    }
}

/// 示例 3: 带返回值的函数
/// 使用 -> 指定返回类型
pub fn function_with_return() {
    println!("\n=== 示例 3: 带返回值的函数 ===");
    
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let product = multiply(4, 7);
    println!("4 × 7 = {}", product);
    
    let is_adult = check_age(20);
    println!("20岁是成年人吗？{}", is_adult);
    
    fn add(a: i32, b: i32) -> i32 {
        a + b  // 最后一个表达式作为返回值（没有分号）
    }
    
    fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }
    
    fn check_age(age: u32) -> bool {
        age >= 18
    }
}

/// 示例 4: 语句与表达式
/// Rust 是基于表达式的语言
pub fn statements_vs_expressions() {
    println!("\n=== 示例 4: 语句与表达式 ===");
    
    // 语句不返回值
    let x = 5;  // 这是一个语句
    println!("x = {}", x);
    
    // 表达式会返回值
    let y = {
        let inner = 3;
        inner + 1  // 没有分号，这是表达式
    };
    println!("y = {}", y);
    
    // 函数调用是表达式
    let sum = calculate_sum(10, 20);
    println!("sum = {}", sum);
    
    fn calculate_sum(a: i32, b: i32) -> i32 {
        a + b  // 表达式，返回计算结果
    }
    
    println!("\n注意：表达式末尾不加分号，语句末尾加分号");
}

/// 示例 5: 提前返回
/// 使用 return 关键字提前返回
pub fn early_return() {
    println!("\n=== 示例 5: 提前返回 ===");
    
    println!("绝对值 -5: {}", abs(-5));
    println!("绝对值 10: {}", abs(10));
    
    let result = divide(10, 2);
    println!("10 ÷ 2 = {}", result);
    
    let result2 = divide(10, 0);
    println!("10 ÷ 0 = {}", result2);
    
    fn abs(x: i32) -> i32 {
        if x < 0 {
            return -x;  // 提前返回
        }
        x
    }
    
    fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            println!("错误：除数不能为零");
            return 0;  // 提前返回
        }
        a / b
    }
}

/// 示例 6: 无返回值函数
/// 返回单元类型 ()
pub fn unit_return_type() {
    println!("\n=== 示例 6: 无返回值函数 ===");
    
    print_message("这是一条消息");
    log_info("系统启动");
    
    fn print_message(msg: &str) {
        println!("消息: {}", msg);
        // 隐式返回 ()
    }
    
    fn log_info(info: &str) -> () {  // 显式声明返回 ()
        println!("[INFO] {}", info);
    }
    
    println!("\n不写返回类型等同于返回 ()（单元类型）");
}

/// 示例 7: 函数作为参数
/// 函数可以作为参数传递
pub fn function_as_parameter() {
    println!("\n=== 示例 7: 函数作为参数 ===");
    
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    
    fn apply_operation(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
        op(x, y)
    }
    
    let sum = apply_operation(5, 3, add);
    println!("5 + 3 = {}", sum);
    
    let product = apply_operation(5, 3, multiply);
    println!("5 × 3 = {}", product);
}

/// 示例 8: 返回函数
/// 函数可以返回函数指针
pub fn return_function() {
    println!("\n=== 示例 8: 返回函数 ===");
    
    fn get_operation(op_type: &str) -> fn(i32, i32) -> i32 {
        fn add(a: i32, b: i32) -> i32 { a + b }
        fn subtract(a: i32, b: i32) -> i32 { a - b }
        
        match op_type {
            "add" => add,
            "subtract" => subtract,
            _ => add,
        }
    }
    
    let add_fn = get_operation("add");
    println!("10 + 5 = {}", add_fn(10, 5));
    
    let sub_fn = get_operation("subtract");
    println!("10 - 5 = {}", sub_fn(10, 5));
}

/// 示例 9: 递归函数
/// 函数可以调用自身
pub fn recursive_function() {
    println!("\n=== 示例 9: 递归函数 ===");
    
    fn factorial(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
    
    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }
    
    println!("5 的阶乘: {}", factorial(5));
    println!("斐波那契数列第 10 项: {}", fibonacci(10));
    
    // 尾递归优化示例
    fn factorial_tail(n: u32) -> u32 {
        fn helper(n: u32, acc: u32) -> u32 {
            if n <= 1 {
                acc
            } else {
                helper(n - 1, n * acc)
            }
        }
        helper(n, 1)
    }
    
    println!("5 的阶乘（尾递归）: {}", factorial_tail(5));
}

/// 示例 10: 方法与关联函数
/// 在结构体上定义函数
pub fn methods_and_associated_functions() {
    println!("\n=== 示例 10: 方法与关联函数 ===");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // 关联函数（类似静态方法）
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        
        // 方法（第一个参数是 self）
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }
        
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
    let rect1 = Rectangle::new(30, 50);
    println!("矩形面积: {}", rect1.area());
    println!("矩形周长: {}", rect1.perimeter());
    
    let rect2 = Rectangle::new(10, 20);
    println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));
}

/// 示例 11: 泛型函数
/// 函数可以使用泛型参数
pub fn generic_functions() {
    println!("\n=== 示例 11: 泛型函数 ===");
    
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大的数字: {}", largest(&numbers));
    
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大的字符: {}", largest(&chars));
}

/// 示例 12: 发散函数
/// 永不返回的函数使用 ! 作为返回类型
pub fn diverging_functions() {
    println!("\n=== 示例 12: 发散函数 ===");
    
    fn get_value(should_panic: bool) -> i32 {
        if should_panic {
            // panic! 是发散函数
            // panic!("程序崩溃！");
            println!("这里本应 panic，但为了演示我们跳过");
            return -1;
        }
        42
    }
    
    println!("正常返回: {}", get_value(false));
    println!("发散函数永不返回，类型为 !");
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 函数定义与调用教学代码         ║");
    println!("╚════════════════════════════════════════╝");
    
    basic_function();
    function_with_parameters();
    function_with_return();
    statements_vs_expressions();
    early_return();
    unit_return_type();
    function_as_parameter();
    return_function();
    recursive_function();
    methods_and_associated_functions();
    generic_functions();
    diverging_functions();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  所有示例运行完毕！                  ║");
    println!("╚════════════════════════════════════════╝");
}

