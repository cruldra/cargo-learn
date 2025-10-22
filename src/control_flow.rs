// Rust 控制流教学代码
// 主题：if、loop、while、for、match

/// 示例 1: if 表达式
/// if 在 Rust 中是表达式，可以返回值
pub fn if_expressions() {
    println!("\n=== 示例 1: if 表达式 ===");
    
    let number = 7;
    
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }
    
    // if 是表达式，可以赋值给变量
    let result = if number % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };
    println!("{} 是 {}", number, result);
    
    // 多个条件
    let score = 85;
    if score >= 90 {
        println!("优秀");
    } else if score >= 80 {
        println!("良好");
    } else if score >= 60 {
        println!("及格");
    } else {
        println!("不及格");
    }
}

/// 示例 2: if let 表达式
/// 简化的模式匹配
pub fn if_let_expressions() {
    println!("\n=== 示例 2: if let 表达式 ===");
    
    let some_value = Some(7);
    
    // 使用 if let 简化 match
    if let Some(x) = some_value {
        println!("值是: {}", x);
    } else {
        println!("没有值");
    }
    
    // 传统的 match 写法
    match some_value {
        Some(x) => println!("使用 match: {}", x),
        None => println!("没有值"),
    }
    
    println!("if let 适合只关心一种情况的场景");
}

/// 示例 3: loop 无限循环
/// loop 创建一个无限循环
pub fn loop_infinite() {
    println!("\n=== 示例 3: loop 无限循环 ===");
    
    let mut counter = 0;
    
    loop {
        counter += 1;
        println!("循环次数: {}", counter);
        
        if counter >= 3 {
            break;  // 使用 break 退出循环
        }
    }
    
    println!("循环结束");
}

/// 示例 4: loop 返回值
/// loop 可以通过 break 返回值
pub fn loop_with_return() {
    println!("\n=== 示例 4: loop 返回值 ===");
    
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // break 可以返回值
        }
    };
    
    println!("结果: {}", result);
}

/// 示例 5: 循环标签
/// 使用标签区分嵌套循环
pub fn loop_labels() {
    println!("\n=== 示例 5: 循环标签 ===");
    
    let mut count = 0;
    
    'outer: loop {
        println!("外层循环 count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("  内层循环 remaining = {}", remaining);
            
            if remaining == 9 {
                break;  // 退出内层循环
            }
            
            if count == 2 {
                break 'outer;  // 退出外层循环
            }
            
            remaining -= 1;
        }
        
        count += 1;
    }
    
    println!("循环结束");
}

/// 示例 6: while 条件循环
/// while 在条件为真时循环
pub fn while_loops() {
    println!("\n=== 示例 6: while 条件循环 ===");
    
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("发射！");
    
    // 使用 while 遍历数组（不推荐）
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < arr.len() {
        println!("数组元素: {}", arr[index]);
        index += 1;
    }
}

/// 示例 7: while let 循环
/// 结合模式匹配的 while 循环
pub fn while_let_loops() {
    println!("\n=== 示例 7: while let 循环 ===");
    
    let mut stack = vec![1, 2, 3, 4, 5];
    
    // 当 pop 返回 Some 时继续循环
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
    
    println!("栈已清空");
}

/// 示例 8: for 循环遍历集合
/// for 是遍历集合的最佳方式
pub fn for_loops() {
    println!("\n=== 示例 8: for 循环遍历集合 ===");
    
    let arr = [10, 20, 30, 40, 50];
    
    // 遍历数组
    for element in arr {
        println!("元素: {}", element);
    }
    
    // 遍历向量
    let vec = vec!["a", "b", "c"];
    for item in &vec {
        println!("项: {}", item);
    }
    
    // 带索引遍历
    for (index, value) in vec.iter().enumerate() {
        println!("索引 {} 的值: {}", index, value);
    }
}

/// 示例 9: for 循环使用范围
/// 使用范围表达式
pub fn for_with_ranges() {
    println!("\n=== 示例 9: for 循环使用范围 ===");
    
    // 范围 1..5 不包含 5
    println!("1..5 (不包含 5):");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();
    
    // 范围 1..=5 包含 5
    println!("1..=5 (包含 5):");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    
    // 反向范围
    println!("反向 (5 到 1):");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();
}

/// 示例 10: break 和 continue
/// 控制循环流程
pub fn break_and_continue() {
    println!("\n=== 示例 10: break 和 continue ===");
    
    // continue 跳过当前迭代
    println!("跳过偶数:");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;  // 跳过偶数
        }
        print!("{} ", i);
    }
    println!();
    
    // break 提前退出循环
    println!("遇到 5 就停止:");
    for i in 1..=10 {
        if i == 5 {
            break;  // 遇到 5 就退出
        }
        print!("{} ", i);
    }
    println!();
}

/// 示例 11: match 表达式
/// 强大的模式匹配
pub fn match_expressions() {
    println!("\n=== 示例 11: match 表达式 ===");
    
    let number = 3;
    
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 => println!("四"),
        5 => println!("五"),
        _ => println!("其他"),  // _ 是通配符
    }
    
    // match 可以返回值
    let result = match number {
        1 | 2 => "小",      // 多个模式
        3 | 4 | 5 => "中",
        6..=10 => "大",     // 范围模式
        _ => "未知",
    };
    println!("数字 {} 是: {}", number, result);
}

/// 示例 12: match 守卫
/// 在模式中添加额外条件
pub fn match_guards() {
    println!("\n=== 示例 12: match 守卫 ===");
    
    let number = 4;
    
    match number {
        n if n < 0 => println!("负数: {}", n),
        n if n == 0 => println!("零"),
        n if n % 2 == 0 => println!("正偶数: {}", n),
        n => println!("正奇数: {}", n),
    }
    
    // 解构和守卫结合
    let point = (3, 4);
    match point {
        (x, y) if x == y => println!("在对角线上"),
        (x, y) if x > y => println!("x 更大"),
        (x, y) if x < y => println!("y 更大"),
        _ => println!("其他情况"),
    }
}

/// 示例 13: 实际应用 - 猜数字游戏
pub fn practical_guessing_game() {
    println!("\n=== 示例 13: 实际应用 - 猜数字游戏 ===");
    
    let secret_number = 7;
    let guesses = vec![3, 7, 5];
    
    for (attempt, guess) in guesses.iter().enumerate() {
        println!("第 {} 次猜测: {}", attempt + 1, guess);
        
        match guess {
            g if *g < secret_number => println!("太小了！"),
            g if *g > secret_number => println!("太大了！"),
            _ => {
                println!("猜对了！");
                break;
            }
        }
    }
}

/// 示例 14: 实际应用 - 斐波那契数列
pub fn practical_fibonacci() {
    println!("\n=== 示例 14: 实际应用 - 斐波那契数列 ===");
    
    let n = 10;
    let mut a = 0;
    let mut b = 1;
    
    print!("斐波那契数列前 {} 项: ", n);
    for _ in 0..n {
        print!("{} ", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    println!();
}

/// 示例 15: 实际应用 - 九九乘法表
pub fn practical_multiplication_table() {
    println!("\n=== 示例 15: 实际应用 - 九九乘法表 ===");
    
    for i in 1..=9 {
        for j in 1..=i {
            print!("{} × {} = {}\t", j, i, i * j);
        }
        println!();
    }
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 控制流教学代码                 ║");
    println!("╚════════════════════════════════════════╝");
    
    if_expressions();
    if_let_expressions();
    loop_infinite();
    loop_with_return();
    loop_labels();
    while_loops();
    while_let_loops();
    for_loops();
    for_with_ranges();
    break_and_continue();
    match_expressions();
    match_guards();
    practical_guessing_game();
    practical_fibonacci();
    practical_multiplication_table();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  所有示例运行完毕！                  ║");
    println!("╚════════════════════════════════════════╝");
}

