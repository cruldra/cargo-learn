/// Rust 闭包与迭代器教学代码
///
/// 闭包是可以捕获其环境的匿名函数
/// 迭代器是一种惰性求值的序列处理方式

use std::collections::HashMap;

/// 示例 1: 闭包基础语法
///
/// 展示闭包的基本语法和类型推断
pub fn closure_basics() {
    println!("\n=== 示例 1: 闭包基础语法 ===");

    // 最简单的闭包
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));

    // 带类型注解的闭包
    let add_two = |x: i32| -> i32 { x + 2 };
    println!("5 + 2 = {}", add_two(5));

    // 多个参数的闭包
    let multiply = |x, y| x * y;
    println!("3 * 4 = {}", multiply(3, 4));

    // 多行闭包
    let complex = |x: i32| {
        let result = x * 2;
        result + 1
    };
    println!("复杂计算: {}", complex(5));

    // 无参数闭包
    let say_hello = || println!("Hello from closure!");
    say_hello();

    println!("\n闭包语法:");
    println!("  |参数| 表达式");
    println!("  |参数| {{ 语句块 }}");
    println!("  |参数: 类型| -> 返回类型 {{ 语句块 }}");
}

/// 示例 2: 闭包类型推断
///
/// 闭包可以自动推断参数和返回值类型
pub fn closure_type_inference() {
    println!("\n=== 示例 2: 闭包类型推断 ===");

    // 编译器根据第一次使用推断类型
    let example = |x| x;
    
    let s = example(String::from("hello"));
    println!("字符串: {}", s);
    
    // 一旦类型确定，就不能改变
    // let n = example(5); // 错误！类型已经推断为 String

    // 显式类型注解避免歧义
    let add = |x: i32, y: i32| x + y;
    println!("3 + 5 = {}", add(3, 5));

    // 闭包类型是唯一的
    let closure1 = |x: i32| x;
    let closure2 = |x: i32| x;
    // closure1 和 closure2 是不同的类型，即使定义相同
    println!("closure1(5) = {}", closure1(5));
    println!("closure2(10) = {}", closure2(10));

    println!("\n类型推断规则:");
    println!("  - 根据第一次使用推断类型");
    println!("  - 每个闭包都有唯一的匿名类型");
    println!("  - 可以显式指定类型避免歧义");
}

/// 示例 3: 闭包捕获环境 - 不可变借用
///
/// 闭包可以捕获其环境中的变量
pub fn closure_capture_immutable() {
    println!("\n=== 示例 3: 闭包捕获环境 - 不可变借用 ===");

    let x = 10;
    let y = 20;

    // 闭包捕获 x 和 y（不可变借用）
    let print_sum = || {
        println!("x + y = {}", x + y);
    };

    print_sum();
    
    // x 和 y 仍然可以使用
    println!("x = {}, y = {}", x, y);

    // 多次调用闭包
    print_sum();

    println!("\n不可变借用捕获:");
    println!("  - 闭包只读取环境变量");
    println!("  - 可以多次调用闭包");
    println!("  - 原变量仍然可用");
}

/// 示例 4: 闭包捕获环境 - 可变借用
///
/// 闭包可以可变地借用环境变量
pub fn closure_capture_mutable() {
    println!("\n=== 示例 4: 闭包捕获环境 - 可变借用 ===");

    let mut count = 0;

    // 闭包可变借用 count
    let mut increment = || {
        count += 1;
        println!("count = {}", count);
    };

    increment();
    increment();
    increment();

    // 闭包使用完后，count 可以再次使用
    println!("最终 count = {}", count);

    println!("\n可变借用捕获:");
    println!("  - 闭包需要声明为 mut");
    println!("  - 闭包可以修改环境变量");
    println!("  - 闭包使用期间，原变量不可访问");
}

/// 示例 5: 闭包捕获环境 - 获取所有权
///
/// 使用 move 关键字让闭包获取所有权
pub fn closure_capture_move() {
    println!("\n=== 示例 5: 闭包捕获环境 - 获取所有权 ===");

    let s = String::from("hello");

    // move 关键字强制闭包获取所有权
    let print_string = move || {
        println!("字符串: {}", s);
    };

    print_string();
    // println!("{}", s); // 错误！s 的所有权已经移动到闭包

    // move 在多线程中很有用
    let data = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        println!("线程中的数据: {:?}", data);
    });
    handle.join().unwrap();

    println!("\nmove 关键字:");
    println!("  - 强制闭包获取所有权");
    println!("  - 原变量不再可用");
    println!("  - 多线程中必须使用 move");
}

/// 示例 6: Fn、FnMut、FnOnce trait
///
/// 三种闭包 trait 的区别
pub fn closure_traits() {
    println!("\n=== 示例 6: Fn、FnMut、FnOnce trait ===");

    // FnOnce: 消耗捕获的变量，只能调用一次
    let s = String::from("hello");
    let consume = || {
        drop(s); // 消耗 s
    };
    consume();
    // consume(); // 错误！只能调用一次

    // FnMut: 可变借用，可以多次调用
    let mut count = 0;
    let mut increment = || {
        count += 1;
    };
    increment();
    increment();
    println!("count = {}", count);

    // Fn: 不可变借用，可以多次调用
    let x = 10;
    let print_x = || {
        println!("x = {}", x);
    };
    print_x();
    print_x();

    println!("\n闭包 trait 层次:");
    println!("  FnOnce: 所有闭包都实现（至少可调用一次）");
    println!("  FnMut: 不消耗捕获变量的闭包（可多次调用）");
    println!("  Fn: 不修改捕获变量的闭包（可多次调用）");
    println!("\n  Fn ⊂ FnMut ⊂ FnOnce");
}

/// 示例 7: 闭包作为参数
///
/// 函数可以接受闭包作为参数
pub fn closure_as_parameter() {
    println!("\n=== 示例 7: 闭包作为参数 ===");

    // 接受 Fn trait 的函数
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(f(x))
    }

    let double = |x| x * 2;
    let result = apply_twice(double, 5);
    println!("应用两次: {}", result); // (5 * 2) * 2 = 20

    // 接受 FnMut trait 的函数
    fn apply_n_times<F>(mut f: F, mut x: i32, n: usize) -> i32
    where
        F: FnMut(i32) -> i32,
    {
        for _ in 0..n {
            x = f(x);
        }
        x
    }

    let add_one = |x| x + 1;
    let result = apply_n_times(add_one, 0, 5);
    println!("应用 5 次: {}", result);

    println!("\n闭包作为参数:");
    println!("  - 使用泛型和 trait bound");
    println!("  - 根据需求选择 Fn、FnMut 或 FnOnce");
    println!("  - 零成本抽象（编译时单态化）");
}

/// 示例 8: 闭包作为返回值
///
/// 函数可以返回闭包
pub fn closure_as_return() {
    println!("\n=== 示例 8: 闭包作为返回值 ===");

    // 返回闭包需要使用 impl Trait 或 Box
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    let add_5 = make_adder(5);
    println!("10 + 5 = {}", add_5(10));

    let add_100 = make_adder(100);
    println!("10 + 100 = {}", add_100(10));

    // 返回 Box<dyn Fn>（动态分发）
    fn make_multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x * n)
    }

    let mul_3 = make_multiplier(3);
    println!("10 * 3 = {}", mul_3(10));

    println!("\n返回闭包:");
    println!("  - impl Trait: 静态分发，性能更好");
    println!("  - Box<dyn Trait>: 动态分发，更灵活");
    println!("  - 必须使用 move 捕获环境");
}

/// 示例 9: 迭代器基础
///
/// Iterator trait 的基本使用
pub fn iterator_basics() {
    println!("\n=== 示例 9: 迭代器基础 ===");

    let v = vec![1, 2, 3, 4, 5];

    // 创建迭代器
    let mut iter = v.iter();

    // 手动调用 next
    println!("第一个元素: {:?}", iter.next());
    println!("第二个元素: {:?}", iter.next());

    // for 循环自动调用 next
    for val in v.iter() {
        println!("值: {}", val);
    }

    // 三种迭代器方法
    let v2 = vec![1, 2, 3];
    
    // iter() - 不可变引用
    for val in v2.iter() {
        println!("不可变引用: {}", val);
    }

    // iter_mut() - 可变引用
    let mut v3 = vec![1, 2, 3];
    for val in v3.iter_mut() {
        *val *= 2;
    }
    println!("修改后: {:?}", v3);

    // into_iter() - 获取所有权
    let v4 = vec![1, 2, 3];
    for val in v4.into_iter() {
        println!("获取所有权: {}", val);
    }
    // println!("{:?}", v4); // 错误！v4 已被消耗

    println!("\n迭代器方法:");
    println!("  iter(): 不可变引用迭代");
    println!("  iter_mut(): 可变引用迭代");
    println!("  into_iter(): 获取所有权迭代");
}

/// 示例 10: 迭代器适配器 - map
///
/// map 转换迭代器中的每个元素
pub fn iterator_map() {
    println!("\n=== 示例 10: 迭代器适配器 - map ===");

    let v = vec![1, 2, 3, 4, 5];

    // map 转换每个元素
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("翻倍: {:?}", doubled);

    // 链式调用多个 map
    let result: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .map(|x| x + 1)
        .collect();
    println!("翻倍再加一: {:?}", result);

    // map 可以改变类型
    let strings: Vec<String> = v.iter()
        .map(|x| format!("数字: {}", x))
        .collect();
    println!("转换为字符串: {:?}", strings);

    println!("\nmap 适配器:");
    println!("  - 惰性求值（需要 collect 才执行）");
    println!("  - 可以改变元素类型");
    println!("  - 可以链式调用");
}

/// 示例 11: 迭代器适配器 - filter
///
/// filter 过滤迭代器中的元素
pub fn iterator_filter() {
    println!("\n=== 示例 11: 迭代器适配器 - filter ===");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 过滤偶数
    let evens: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .copied()
        .collect();
    println!("偶数: {:?}", evens);

    // 过滤大于 5 的数
    let greater_than_5: Vec<i32> = v.iter()
        .filter(|x| **x > 5)
        .copied()
        .collect();
    println!("大于 5: {:?}", greater_than_5);

    // 组合 filter 和 map
    let result: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("偶数翻倍: {:?}", result);

    println!("\nfilter 适配器:");
    println!("  - 根据条件过滤元素");
    println!("  - 闭包返回 bool");
    println!("  - 可以与其他适配器组合");
}

/// 示例 12: 迭代器适配器 - fold
///
/// fold 将迭代器归约为单个值
pub fn iterator_fold() {
    println!("\n=== 示例 12: 迭代器适配器 - fold ===");

    let v = vec![1, 2, 3, 4, 5];

    // 求和
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("求和: {}", sum);

    // 求积
    let product = v.iter().fold(1, |acc, x| acc * x);
    println!("求积: {}", product);

    // 找最大值
    let max = v.iter().fold(i32::MIN, |acc, x| acc.max(*x));
    println!("最大值: {}", max);

    // 构建字符串
    let words = vec!["hello", "world", "rust"];
    let sentence = words.iter().fold(String::new(), |mut acc, word| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(word);
        acc
    });
    println!("句子: {}", sentence);

    println!("\nfold 适配器:");
    println!("  - 归约操作");
    println!("  - 需要初始值和累加函数");
    println!("  - 可以构建任何类型的结果");
}

/// 示例 13: 其他常用迭代器方法
///
/// 展示更多实用的迭代器方法
pub fn iterator_other_methods() {
    println!("\n=== 示例 13: 其他常用迭代器方法 ===");

    let v = vec![1, 2, 3, 4, 5];

    // take - 取前 n 个元素
    let first_three: Vec<i32> = v.iter().take(3).copied().collect();
    println!("前三个: {:?}", first_three);

    // skip - 跳过前 n 个元素
    let skip_two: Vec<i32> = v.iter().skip(2).copied().collect();
    println!("跳过两个: {:?}", skip_two);

    // enumerate - 添加索引
    for (i, val) in v.iter().enumerate() {
        println!("索引 {}: 值 {}", i, val);
    }

    // zip - 组合两个迭代器
    let v2 = vec!["a", "b", "c"];
    let zipped: Vec<(i32, &str)> = v.iter().copied().zip(v2.iter().copied()).collect();
    println!("组合: {:?}", zipped);

    // chain - 连接两个迭代器
    let v3 = vec![6, 7, 8];
    let chained: Vec<i32> = v.iter().chain(v3.iter()).copied().collect();
    println!("连接: {:?}", chained);

    // any - 是否有元素满足条件
    let has_even = v.iter().any(|x| x % 2 == 0);
    println!("有偶数: {}", has_even);

    // all - 是否所有元素满足条件
    let all_positive = v.iter().all(|x| *x > 0);
    println!("都是正数: {}", all_positive);

    // find - 查找第一个满足条件的元素
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("第一个偶数: {:?}", first_even);

    println!("\n常用迭代器方法:");
    println!("  take/skip: 控制数量");
    println!("  enumerate: 添加索引");
    println!("  zip/chain: 组合迭代器");
    println!("  any/all/find: 查询操作");
}

/// 示例 14: 自定义迭代器 - 基础
///
/// 实现 Iterator trait
pub fn custom_iterator_basics() {
    println!("\n=== 示例 14: 自定义迭代器 - 基础 ===");

    // 计数器迭代器
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter::new(5);
    for num in counter {
        println!("计数: {}", num);
    }

    // 使用迭代器适配器
    let sum: u32 = Counter::new(10).sum();
    println!("1 到 10 的和: {}", sum);

    let doubled: Vec<u32> = Counter::new(5).map(|x| x * 2).collect();
    println!("翻倍: {:?}", doubled);

    println!("\n自定义迭代器:");
    println!("  - 实现 Iterator trait");
    println!("  - 定义关联类型 Item");
    println!("  - 实现 next 方法");
    println!("  - 自动获得所有迭代器方法");
}

/// 示例 15: 自定义迭代器 - 范围
///
/// 实现一个范围迭代器
pub fn custom_iterator_range() {
    println!("\n=== 示例 15: 自定义迭代器 - 范围 ===");

    struct StepRange {
        current: i32,
        end: i32,
        step: i32,
    }

    impl StepRange {
        fn new(start: i32, end: i32, step: i32) -> Self {
            StepRange {
                current: start,
                end,
                step,
            }
        }
    }

    impl Iterator for StepRange {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let result = self.current;
                self.current += self.step;
                Some(result)
            } else {
                None
            }
        }
    }

    // 使用自定义范围迭代器
    let range = StepRange::new(0, 20, 3);
    let values: Vec<i32> = range.collect();
    println!("步长为 3 的范围: {:?}", values);

    // 组合使用
    let sum: i32 = StepRange::new(1, 100, 2)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("1-100 中步长为 2 且能被 3 整除的数之和: {}", sum);

    println!("\n自定义范围迭代器:");
    println!("  - 灵活控制迭代逻辑");
    println!("  - 可以与标准适配器组合");
}

/// 示例 16: 实际应用 - 数据处理管道
///
/// 使用迭代器构建数据处理管道
pub fn practical_data_pipeline() {
    println!("\n=== 示例 16: 实际应用 - 数据处理管道 ===");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        salary: u32,
    }

    let people = vec![
        Person { name: "Alice".to_string(), age: 30, salary: 50000 },
        Person { name: "Bob".to_string(), age: 25, salary: 45000 },
        Person { name: "Charlie".to_string(), age: 35, salary: 60000 },
        Person { name: "David".to_string(), age: 28, salary: 48000 },
        Person { name: "Eve".to_string(), age: 32, salary: 55000 },
    ];

    // 找出年龄大于 28 且薪水大于 50000 的人的名字
    let high_earners: Vec<String> = people.iter()
        .filter(|p| p.age > 28)
        .filter(|p| p.salary > 50000)
        .map(|p| p.name.clone())
        .collect();
    println!("高收入者: {:?}", high_earners);

    // 计算平均薪水
    let avg_salary = people.iter()
        .map(|p| p.salary)
        .sum::<u32>() as f64 / people.len() as f64;
    println!("平均薪水: {:.2}", avg_salary);

    // 按年龄分组统计
    let mut age_groups: HashMap<u32, Vec<String>> = HashMap::new();
    for person in &people {
        age_groups.entry(person.age / 10 * 10)
            .or_insert_with(Vec::new)
            .push(person.name.clone());
    }
    println!("年龄分组: {:?}", age_groups);

    println!("\n数据处理管道:");
    println!("  - 链式调用多个操作");
    println!("  - 声明式编程风格");
    println!("  - 惰性求值，高效执行");
}

/// 示例 17: 实际应用 - 文本处理
///
/// 使用迭代器处理文本
pub fn practical_text_processing() {
    println!("\n=== 示例 17: 实际应用 - 文本处理 ===");

    let text = "Hello World! This is a test. Rust is awesome!";

    // 统计单词数
    let word_count = text.split_whitespace().count();
    println!("单词数: {}", word_count);

    // 找出最长的单词
    let longest = text.split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap();
    println!("最长的单词: {}", longest);

    // 转换为大写并收集
    let uppercase: Vec<String> = text.split_whitespace()
        .map(|word| word.to_uppercase())
        .collect();
    println!("大写: {:?}", uppercase);

    // 过滤并统计长度大于 4 的单词
    let long_words: Vec<&str> = text.split_whitespace()
        .filter(|word| word.len() > 4)
        .collect();
    println!("长单词: {:?}", long_words);

    // 构建单词频率表
    let mut word_freq: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *word_freq.entry(word).or_insert(0) += 1;
    }
    println!("单词频率: {:?}", word_freq);

    println!("\n文本处理:");
    println!("  - split_whitespace 分割单词");
    println!("  - 组合多个迭代器操作");
    println!("  - 高效且易读");
}

/// 示例 18: 实际应用 - 惰性求值优化
///
/// 展示迭代器的惰性求值特性
pub fn practical_lazy_evaluation() {
    println!("\n=== 示例 18: 实际应用 - 惰性求值优化 ===");

    let v: Vec<i32> = (1..=1000000).collect();

    // 惰性求值：只处理需要的元素
    let result: Vec<i32> = v.iter()
        .filter(|x| {
            // 这个闭包只会被调用很少次
            *x % 2 == 0
        })
        .take(5) // 只取前 5 个
        .copied()
        .collect();
    println!("前 5 个偶数: {:?}", result);

    // 对比：如果不使用惰性求值
    // 这会处理所有 100 万个元素
    let all_evens: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .copied()
        .collect();
    println!("所有偶数的数量: {}", all_evens.len());

    // find 也是惰性的，找到第一个就停止
    let first_divisible_by_7 = v.iter()
        .find(|x| *x % 7 == 0);
    println!("第一个能被 7 整除的数: {:?}", first_divisible_by_7);

    println!("\n惰性求值的优势:");
    println!("  - 只处理需要的元素");
    println!("  - 避免不必要的计算");
    println!("  - 可以处理无限序列");
    println!("  - 内存效率高");
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Rust 闭包与迭代器教学代码            ║");
    println!("╚════════════════════════════════════════╝");

    closure_basics();
    closure_type_inference();
    closure_capture_immutable();
    closure_capture_mutable();
    closure_capture_move();
    closure_traits();
    closure_as_parameter();
    closure_as_return();
    iterator_basics();
    iterator_map();
    iterator_filter();
    iterator_fold();
    iterator_other_methods();
    custom_iterator_basics();
    custom_iterator_range();
    practical_data_pipeline();
    practical_text_processing();
    practical_lazy_evaluation();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  闭包和迭代器是 Rust 的强大特性！    ║");
    println!("╚════════════════════════════════════════╝\n");
}

