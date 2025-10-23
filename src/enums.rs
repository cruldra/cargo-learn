// Rust 枚举教学代码
// 主题：枚举定义、模式匹配、Option、Result

/// 示例 1: 基本枚举定义
pub fn basic_enum() {
    println!("\n=== 示例 1: 基本枚举定义 ===");
    
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    let dir = Direction::North;
    
    match dir {
        Direction::North => println!("向北"),
        Direction::South => println!("向南"),
        Direction::East => println!("向东"),
        Direction::West => println!("向西"),
    }
    
    println!("枚举用于表示一组固定的可能值");
}

/// 示例 2: 带数据的枚举
pub fn enum_with_data() {
    println!("\n=== 示例 2: 带数据的枚举 ===");
    
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    match msg1 {
        Message::Quit => println!("退出消息"),
        _ => println!("其他消息"),
    }
    
    match msg2 {
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        _ => println!("其他消息"),
    }
    
    match msg3 {
        Message::Write(text) => println!("写入: {}", text),
        _ => println!("其他消息"),
    }
    
    match msg4 {
        Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        _ => println!("其他消息"),
    }
    
    println!("枚举的每个变体可以携带不同类型和数量的数据");
}

/// 示例 3: 枚举方法
pub fn enum_methods() {
    println!("\n=== 示例 3: 枚举方法 ===");
    
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }
    
    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("执行退出操作"),
                Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
                Message::Write(text) => println!("写入: {}", text),
            }
        }
        
        fn describe(&self) -> String {
            match self {
                Message::Quit => String::from("退出消息"),
                Message::Move { .. } => String::from("移动消息"),
                Message::Write(_) => String::from("写入消息"),
            }
        }
    }
    
    let msg = Message::Write(String::from("Hello, Rust!"));
    msg.call();
    println!("消息类型: {}", msg.describe());
}

/// 示例 4: Option 枚举
pub fn option_enum() {
    println!("\n=== 示例 4: Option 枚举 ===");
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    
    match some_number {
        Some(value) => println!("有值: {}", value),
        None => println!("没有值"),
    }
    
    match absent_number {
        Some(value) => println!("有值: {}", value),
        None => println!("没有值"),
    }
    
    println!("Option 用于表示可能存在或不存在的值，避免空指针");
}

/// 示例 5: Option 的方法
pub fn option_methods() {
    println!("\n=== 示例 5: Option 的方法 ===");
    
    let x = Some(5);
    let y: Option<i32> = None;
    
    println!("x.is_some(): {}", x.is_some());
    println!("x.is_none(): {}", x.is_none());
    println!("y.is_some(): {}", y.is_some());
    println!("y.is_none(): {}", y.is_none());
    
    println!("x.unwrap(): {}", x.unwrap());
    // println!("y.unwrap(): {}", y.unwrap());  // 会 panic!
    
    println!("x.unwrap_or(0): {}", x.unwrap_or(0));
    println!("y.unwrap_or(0): {}", y.unwrap_or(0));
    
    println!("x.unwrap_or_default(): {}", x.unwrap_or_default());
    println!("y.unwrap_or_default(): {}", y.unwrap_or_default());
    
    let doubled = x.map(|v| v * 2);
    println!("x.map(|v| v * 2): {:?}", doubled);
    
    let doubled_none = y.map(|v| v * 2);
    println!("y.map(|v| v * 2): {:?}", doubled_none);
}

/// 示例 6: if let 简化匹配
pub fn if_let_pattern() {
    println!("\n=== 示例 6: if let 简化匹配 ===");
    
    let some_value = Some(3);
    
    match some_value {
        Some(3) => println!("匹配到 3"),
        _ => (),
    }
    
    if let Some(3) = some_value {
        println!("if let: 匹配到 3");
    }
    
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("使用你最喜欢的颜色: {}", color);
    } else if is_tuesday {
        println!("星期二是绿色的日子");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色作为背景色");
        } else {
            println!("使用橙色作为背景色");
        }
    } else {
        println!("使用蓝色作为背景色");
    }
}

/// 示例 7: while let 循环
pub fn while_let_pattern() {
    println!("\n=== 示例 7: while let 循环 ===");
    
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("弹出栈中的元素:");
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    println!("栈已空");
}

/// 示例 8: Result 枚举
pub fn result_enum() {
    println!("\n=== 示例 8: Result 枚举 ===");
    
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(a / b)
        }
    }
    
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);
    
    match result1 {
        Ok(value) => println!("10.0 / 2.0 = {}", value),
        Err(e) => println!("错误: {}", e),
    }
    
    match result2 {
        Ok(value) => println!("10.0 / 0.0 = {}", value),
        Err(e) => println!("错误: {}", e),
    }
    
    println!("Result 用于表示可能成功或失败的操作");
}

/// 示例 9: Result 的方法
pub fn result_methods() {
    println!("\n=== 示例 9: Result 的方法 ===");
    
    let good_result: Result<i32, &str> = Ok(10);
    let bad_result: Result<i32, &str> = Err("出错了");
    
    println!("good_result.is_ok(): {}", good_result.is_ok());
    println!("good_result.is_err(): {}", good_result.is_err());
    println!("bad_result.is_ok(): {}", bad_result.is_ok());
    println!("bad_result.is_err(): {}", bad_result.is_err());
    
    println!("good_result.unwrap(): {}", good_result.unwrap());
    // println!("bad_result.unwrap(): {}", bad_result.unwrap());  // 会 panic!
    
    println!("good_result.unwrap_or(0): {}", good_result.unwrap_or(0));
    println!("bad_result.unwrap_or(0): {}", bad_result.unwrap_or(0));
    
    let doubled = good_result.map(|v| v * 2);
    println!("good_result.map(|v| v * 2): {:?}", doubled);
    
    let error_mapped = bad_result.map_err(|e| format!("错误: {}", e));
    println!("bad_result.map_err(...): {:?}", error_mapped);
}

/// 示例 10: 模式匹配的强大功能
pub fn pattern_matching_power() {
    println!("\n=== 示例 10: 模式匹配的强大功能 ===");

    enum Color {
        Rgb(u8, u8, u8),
        Hsv(u8, u8, u8),
    }

    let color = Color::Rgb(255, 0, 0);

    match color {
        Color::Rgb(255, 0, 0) => println!("纯红色"),
        Color::Rgb(0, 255, 0) => println!("纯绿色"),
        Color::Rgb(0, 0, 255) => println!("纯蓝色"),
        Color::Rgb(r, g, b) => println!("其他 RGB 颜色: ({}, {}, {})", r, g, b),
        Color::Hsv(h, s, v) => println!("HSV 颜色: ({}, {}, {})", h, s, v),
    }

    let number = 13;
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 => println!("质数"),
        13..=19 => println!("十几"),
        _ => println!("其他数字"),
    }
}

/// 示例 11: 枚举与结构体结合
pub fn enum_with_struct() {
    println!("\n=== 示例 11: 枚举与结构体结合 ===");

    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[derive(Debug)]
    enum Shape {
        Circle { center: Point, radius: f64 },
        Rectangle { top_left: Point, bottom_right: Point },
        Triangle { p1: Point, p2: Point, p3: Point },
    }

    impl Shape {
        fn describe(&self) -> String {
            match self {
                Shape::Circle { center, radius } => {
                    format!("圆心在 ({}, {})，半径为 {}", center.x, center.y, radius)
                }
                Shape::Rectangle { top_left, bottom_right } => {
                    format!("矩形从 ({}, {}) 到 ({}, {})",
                        top_left.x, top_left.y, bottom_right.x, bottom_right.y)
                }
                Shape::Triangle { p1, p2, p3 } => {
                    format!("三角形顶点: ({}, {}), ({}, {}), ({}, {})",
                        p1.x, p1.y, p2.x, p2.y, p3.x, p3.y)
                }
            }
        }
    }

    let circle = Shape::Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 5.0,
    };

    let rect = Shape::Rectangle {
        top_left: Point { x: 0.0, y: 10.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };

    println!("{}", circle.describe());
    println!("{}", rect.describe());
}

/// 示例 12: 递归枚举
pub fn recursive_enum() {
    println!("\n=== 示例 12: 递归枚举 ===");

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    fn print_list(list: &List) {
        match list {
            Cons(value, next) => {
                print!("{} -> ", value);
                print_list(next);
            }
            Nil => println!("Nil"),
        }
    }

    print!("链表: ");
    print_list(&list);
    println!("递归枚举需要使用 Box 来避免无限大小");
}

/// 示例 13: 枚举的内存布局
pub fn enum_memory_layout() {
    println!("\n=== 示例 13: 枚举的内存布局 ===");

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    println!("Message 枚举大小: {} 字节", std::mem::size_of::<Message>());
    println!("Option<i32> 大小: {} 字节", std::mem::size_of::<Option<i32>>());
    println!("Option<&i32> 大小: {} 字节", std::mem::size_of::<Option<&i32>>());
    println!("Result<i32, String> 大小: {} 字节", std::mem::size_of::<Result<i32, String>>());

    println!("枚举的大小等于最大变体的大小加上判别式的大小");
}

/// 示例 14: 实际应用 - 状态机
pub fn practical_state_machine() {
    println!("\n=== 示例 14: 实际应用 - 状态机 ===");

    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight {
        fn next(&self) -> TrafficLight {
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Yellow => TrafficLight::Red,
                TrafficLight::Green => TrafficLight::Yellow,
            }
        }

        fn duration(&self) -> u32 {
            match self {
                TrafficLight::Red => 60,
                TrafficLight::Yellow => 3,
                TrafficLight::Green => 55,
            }
        }

        fn action(&self) -> &str {
            match self {
                TrafficLight::Red => "停止",
                TrafficLight::Yellow => "准备",
                TrafficLight::Green => "通行",
            }
        }
    }

    let mut light = TrafficLight::Red;

    for _ in 0..5 {
        println!("{:?} 灯 - {} (持续 {} 秒)", light, light.action(), light.duration());
        light = light.next();
    }
}

/// 示例 15: 实际应用 - 表达式求值
pub fn practical_expression_eval() {
    println!("\n=== 示例 15: 实际应用 - 表达式求值 ===");

    #[derive(Debug)]
    enum Expr {
        Number(f64),
        Add(Box<Expr>, Box<Expr>),
        Subtract(Box<Expr>, Box<Expr>),
        Multiply(Box<Expr>, Box<Expr>),
        Divide(Box<Expr>, Box<Expr>),
    }

    impl Expr {
        fn eval(&self) -> f64 {
            match self {
                Expr::Number(n) => *n,
                Expr::Add(left, right) => left.eval() + right.eval(),
                Expr::Subtract(left, right) => left.eval() - right.eval(),
                Expr::Multiply(left, right) => left.eval() * right.eval(),
                Expr::Divide(left, right) => left.eval() / right.eval(),
            }
        }

        fn to_string(&self) -> String {
            match self {
                Expr::Number(n) => n.to_string(),
                Expr::Add(left, right) => format!("({} + {})", left.to_string(), right.to_string()),
                Expr::Subtract(left, right) => format!("({} - {})", left.to_string(), right.to_string()),
                Expr::Multiply(left, right) => format!("({} * {})", left.to_string(), right.to_string()),
                Expr::Divide(left, right) => format!("({} / {})", left.to_string(), right.to_string()),
            }
        }
    }

    let expr = Expr::Add(
        Box::new(Expr::Multiply(
            Box::new(Expr::Number(2.0)),
            Box::new(Expr::Number(3.0))
        )),
        Box::new(Expr::Divide(
            Box::new(Expr::Number(10.0)),
            Box::new(Expr::Number(2.0))
        ))
    );

    println!("表达式: {}", expr.to_string());
    println!("结果: {}", expr.eval());
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 枚举教学代码                   ║");
    println!("╚════════════════════════════════════════╝");

    basic_enum();
    enum_with_data();
    enum_methods();
    option_enum();
    option_methods();
    if_let_pattern();
    while_let_pattern();
    result_enum();
    result_methods();
    pattern_matching_power();
    enum_with_struct();
    recursive_enum();
    enum_memory_layout();
    practical_state_machine();
    practical_expression_eval();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  枚举是 Rust 中表达复杂数据的利器！ ║");
    println!("╚════════════════════════════════════════╝");
}


