//! # 生命周期（Lifetimes）
//!
//! 生命周期是 Rust 最独特的特性之一，用于确保引用始终有效。
//! 生命周期注解描述了多个引用之间的关系，帮助编译器验证引用的有效性。

use std::fmt::Display;

/// 示例 1: 生命周期问题演示
///
/// 展示为什么需要生命周期注解
pub fn lifetime_problem() {
    println!("\n=== 示例 1: 生命周期问题演示 ===");

    // 这个函数无法编译，因为编译器不知道返回的引用来自哪个参数
    // fn longest(x: &str, y: &str) -> &str {  // ❌ 缺少生命周期注解
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // 正确的版本：添加生命周期注解
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // 'a 表示一个生命周期参数
        // 返回值的生命周期与两个参数中较短的那个相同
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("最长的字符串是: {}", result);

    println!("生命周期注解告诉编译器引用之间的关系");
}

/// 示例 2: 生命周期注解语法
///
/// 详细解释生命周期注解的语法
pub fn lifetime_syntax() {
    println!("\n=== 示例 2: 生命周期注解语法 ===");

    // 生命周期注解语法：
    // &i32        - 引用
    // &'a i32     - 带有显式生命周期的引用
    // &'a mut i32 - 带有显式生命周期的可变引用

    // 生命周期参数以 ' 开头，通常使用短小的名字
    // 'a, 'b, 'c 是最常见的
    // 'static 是特殊的生命周期，表示整个程序运行期间

    fn example<'a>(x: &'a i32) -> &'a i32 {
        // 'a 是生命周期参数
        // x 的生命周期是 'a
        // 返回值的生命周期也是 'a
        x
    }

    let num = 42;
    let result = example(&num);
    println!("结果: {}", result);

    println!("\n生命周期注解语法:");
    println!("  &i32        - 普通引用");
    println!("  &'a i32     - 带生命周期的引用");
    println!("  &'a mut i32 - 带生命周期的可变引用");
}

/// 示例 3: 函数中的生命周期
///
/// 展示函数参数和返回值的生命周期关系
pub fn lifetime_in_functions() {
    println!("\n=== 示例 3: 函数中的生命周期 ===");

    // 返回值的生命周期与参数相关
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // 返回值的生命周期只与一个参数相关
    fn first<'a>(x: &'a str, y: &str) -> &'a str {
        // y 的生命周期不影响返回值
        x
    }

    // 多个不同的生命周期参数
    fn announce<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
    where
        'b: 'a, // 'b 的生命周期至少和 'a 一样长
    {
        println!("注意: {}", y);
        x
    }

    let s1 = String::from("hello");
    let s2 = String::from("world");

    let result1 = longest(&s1, &s2);
    println!("最长的: {}", result1);

    let result2 = first(&s1, &s2);
    println!("第一个: {}", result2);

    let result3 = announce(&s1, &s2);
    println!("返回: {}", result3);

    println!("函数的生命周期注解描述了参数和返回值之间的关系");
}

/// 示例 4: 生命周期约束
///
/// 展示生命周期如何约束引用的有效性
pub fn lifetime_constraints() {
    println!("\n=== 示例 4: 生命周期约束 ===");

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // 有效的使用
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("最长的字符串是: {}", result); // ✅ 在 string2 的作用域内使用
    }
    // println!("{}", result); // ❌ 这里会报错，因为 string2 已经被释放

    // 生命周期确保引用不会超过被引用数据的生命周期
    println!("生命周期防止悬垂引用");
}

/// 示例 5: 结构体中的生命周期
///
/// 展示如何在结构体中使用生命周期注解
pub fn lifetime_in_structs() {
    println!("\n=== 示例 5: 结构体中的生命周期 ===");

    // 结构体包含引用时，必须添加生命周期注解
    struct ImportantExcerpt<'a> {
        part: &'a str, // part 是一个字符串切片的引用
    }

    // 实现方法
    impl<'a> ImportantExcerpt<'a> {
        // 方法的生命周期注解
        fn level(&self) -> i32 {
            3
        }

        // 返回引用的方法
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("请注意: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("找不到 '.'");

    // 创建结构体实例
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("摘录: {}", excerpt.part);
    println!("级别: {}", excerpt.level());
    println!(
        "返回: {}",
        excerpt.announce_and_return_part("这是重要的部分")
    );

    println!("\n结构体的生命周期注解确保:");
    println!("  - 结构体实例的生命周期不能超过其引用的数据");
    println!("  - ImportantExcerpt 的实例不能比 part 引用的数据活得更久");
}

/// 示例 6: 生命周期省略规则
///
/// 展示编译器自动推断生命周期的规则
pub fn lifetime_elision() {
    println!("\n=== 示例 6: 生命周期省略规则 ===");

    // 规则 1: 每个引用参数都有自己的生命周期
    fn first_word(s: &str) -> &str {
        // 实际上是: fn first_word<'a>(s: &'a str) -> &'a str
        s.split_whitespace().next().unwrap_or("")
    }

    // 规则 2: 如果只有一个输入生命周期，它被赋予所有输出生命周期
    fn get_first(s: &str) -> &str {
        // 实际上是: fn get_first<'a>(s: &'a str) -> &'a str
        &s[0..1]
    }

    // 规则 3: 如果有多个输入生命周期，但其中一个是 &self 或 &mut self
    // 那么 self 的生命周期被赋予所有输出生命周期
    struct Parser<'a> {
        data: &'a str,
    }

    impl<'a> Parser<'a> {
        fn parse(&self) -> &str {
            // 实际上是: fn parse(&self) -> &'a str
            // 返回值的生命周期与 self 相同
            self.data
        }
    }

    let text = String::from("hello world");
    println!("第一个单词: {}", first_word(&text));
    println!("第一个字符: {}", get_first(&text));

    let parser = Parser { data: &text };
    println!("解析结果: {}", parser.parse());

    println!("\n生命周期省略规则让代码更简洁:");
    println!("  1. 每个引用参数都有自己的生命周期");
    println!("  2. 单个输入生命周期赋予所有输出");
    println!("  3. 方法中 self 的生命周期赋予所有输出");
}

/// 示例 7: 静态生命周期
///
/// 展示 'static 生命周期的使用
pub fn static_lifetime() {
    println!("\n=== 示例 7: 静态生命周期 ===");

    // 'static 表示引用在整个程序运行期间都有效
    let s: &'static str = "我是静态字符串";
    println!("静态字符串: {}", s);

    // 字符串字面量都有 'static 生命周期
    // 因为它们被直接存储在程序的二进制文件中
    let static_str: &'static str = "hello";

    // 使用 'static 的函数
    fn get_static() -> &'static str {
        "这是一个静态字符串"
    }

    println!("静态函数返回: {}", get_static());

    // 注意: 不要滥用 'static
    // 大多数情况下，生命周期问题应该通过正确的生命周期注解解决
    // 而不是使用 'static

    println!("\n'static 生命周期:");
    println!("  - 表示引用在整个程序运行期间都有效");
    println!("  - 字符串字面量默认是 'static");
    println!("  - 不要滥用，大多数情况应该使用正确的生命周期注解");
}

/// 示例 8: 生命周期与泛型
///
/// 展示生命周期、泛型和 trait bound 的组合使用
pub fn lifetime_with_generics() {
    println!("\n=== 示例 8: 生命周期与泛型 ===");

    // 同时使用生命周期、泛型和 trait bound
    fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("公告: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let announcement = "这是一个重要的比较";

    let result = longest_with_announcement(&s1, &s2, announcement);
    println!("最长的字符串: {}", result);

    println!("\n可以同时使用:");
    println!("  - 生命周期参数 ('a)");
    println!("  - 泛型类型参数 (T)");
    println!("  - Trait bound (T: Display)");
}

/// 示例 9: 多个生命周期参数
///
/// 展示如何使用多个不同的生命周期参数
pub fn multiple_lifetimes() {
    println!("\n=== 示例 9: 多个生命周期参数 ===");

    // 两个不同的生命周期参数
    fn first_part<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        // 返回值的生命周期只与 x 相关
        println!("y 的值是: {}", y);
        x
    }

    // 生命周期约束: 'b 必须至少和 'a 一样长
    fn constrained<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
        // 'b: 'a 表示 'b 的生命周期至少和 'a 一样长
        println!("使用 y: {}", y);
        x
    }

    let string1 = String::from("hello");
    let string2 = String::from("world");

    let result1 = first_part(&string1, &string2);
    println!("返回第一个: {}", result1);

    let result2 = constrained(&string1, &string2);
    println!("带约束的返回: {}", result2);

    println!("\n多个生命周期参数:");
    println!("  - 'a, 'b 表示不同的生命周期");
    println!("  - 'b: 'a 表示 'b 至少和 'a 一样长");
}

/// 示例 10: 结构体方法中的生命周期
///
/// 展示结构体方法中生命周期的详细用法
pub fn lifetime_in_methods() {
    println!("\n=== 示例 10: 结构体方法中的生命周期 ===");

    struct Context<'a> {
        text: &'a str,
    }

    impl<'a> Context<'a> {
        // 方法不需要额外的生命周期注解（省略规则）
        fn get_text(&self) -> &str {
            self.text
        }

        // 方法有额外的引用参数
        fn compare(&self, other: &str) -> &'a str {
            // 返回值的生命周期与 self 相同（省略规则 3）
            // 但这里我们总是返回 self.text，所以生命周期是 'a
            if self.text.len() > other.len() {
                self.text
            } else {
                // 注意：这里实际上不能返回 other，因为它的生命周期可能比 'a 短
                // 所以我们只返回 self.text
                self.text
            }
        }

        // 返回新的引用，生命周期与输入参数相关
        fn get_part<'b>(&self, part: &'b str) -> &'b str {
            part
        }
    }

    let text = String::from("这是上下文文本");
    let ctx = Context { text: &text };

    println!("文本: {}", ctx.get_text());

    let other_text = String::from("短");
    println!("比较: {}", ctx.compare(&other_text));

    let part = String::from("新部分");
    println!("部分: {}", ctx.get_part(&part));

    println!("\n方法中的生命周期:");
    println!("  - 通常可以省略（省略规则）");
    println!("  - 返回值默认与 self 的生命周期相同");
    println!("  - 可以有额外的生命周期参数");
}

/// 示例 11: 生命周期子类型
///
/// 展示生命周期的协变和子类型关系
pub fn lifetime_subtyping() {
    println!("\n=== 示例 11: 生命周期子类型 ===");

    // 较长的生命周期可以转换为较短的生命周期
    fn choose<'a, 'b>(first: &'a str, second: &'b str, use_first: bool) -> &'a str
    where
        'b: 'a, // 'b 的生命周期至少和 'a 一样长
    {
        if use_first {
            first
        } else {
            // 可以返回 'b，因为 'b: 'a
            second
        }
    }

    let long_string = String::from("长生命周期的字符串");
    let result;

    {
        let short_string = String::from("短");
        result = choose(&long_string, &short_string, false);
        println!("选择的字符串: {}", result);
    }
    // result 的生命周期受限于较短的那个

    println!("\n生命周期子类型:");
    println!("  - 'b: 'a 表示 'b 至少和 'a 一样长");
    println!("  - 较长的生命周期可以用在需要较短生命周期的地方");
}

/// 示例 12: 实际应用 - 字符串解析器
///
/// 使用生命周期实现一个字符串解析器
pub fn practical_parser() {
    println!("\n=== 示例 12: 实际应用 - 字符串解析器 ===");

    // 解析器结构体，持有对原始数据的引用
    struct Parser<'a> {
        data: &'a str,
        position: usize,
    }

    impl<'a> Parser<'a> {
        fn new(data: &'a str) -> Self {
            Parser { data, position: 0 }
        }

        // 读取下一个单词
        fn next_word(&mut self) -> Option<&'a str> {
            // 跳过空白字符
            while self.position < self.data.len() {
                if !self.data[self.position..].starts_with(' ') {
                    break;
                }
                self.position += 1;
            }

            if self.position >= self.data.len() {
                return None;
            }

            // 找到单词的结束位置
            let start = self.position;
            while self.position < self.data.len()
                && !self.data[self.position..].starts_with(' ')
            {
                self.position += 1;
            }

            Some(&self.data[start..self.position])
        }

        // 获取剩余的文本
        fn remaining(&self) -> &'a str {
            &self.data[self.position..]
        }
    }

    let text = String::from("hello world rust programming");
    let mut parser = Parser::new(&text);

    println!("解析文本: {}", text);
    while let Some(word) = parser.next_word() {
        println!("  单词: {}", word);
    }

    println!("\n解析器使用生命周期:");
    println!("  - 避免复制数据");
    println!("  - 返回原始字符串的切片");
    println!("  - 确保引用的有效性");
}

/// 示例 13: 实际应用 - 配置管理器
///
/// 使用生命周期管理配置引用
pub fn practical_config() {
    println!("\n=== 示例 13: 实际应用 - 配置管理器 ===");

    // 配置项
    struct Config<'a> {
        name: &'a str,
        value: &'a str,
    }

    // 配置管理器
    struct ConfigManager<'a> {
        configs: Vec<Config<'a>>,
    }

    impl<'a> ConfigManager<'a> {
        fn new() -> Self {
            ConfigManager {
                configs: Vec::new(),
            }
        }

        fn add(&mut self, name: &'a str, value: &'a str) {
            self.configs.push(Config { name, value });
        }

        fn get(&self, name: &str) -> Option<&'a str> {
            self.configs
                .iter()
                .find(|c| c.name == name)
                .map(|c| c.value)
        }

        fn list(&self) {
            for config in &self.configs {
                println!("  {} = {}", config.name, config.value);
            }
        }
    }

    let app_name = String::from("MyApp");
    let version = String::from("1.0.0");
    let author = String::from("Rust Developer");

    let mut manager = ConfigManager::new();
    manager.add("app_name", &app_name);
    manager.add("version", &version);
    manager.add("author", &author);

    println!("配置列表:");
    manager.list();

    if let Some(name) = manager.get("app_name") {
        println!("\n应用名称: {}", name);
    }

    println!("\n配置管理器的优势:");
    println!("  - 不复制配置数据");
    println!("  - 生命周期确保配置数据的有效性");
    println!("  - 高效的内存使用");
}

/// 示例 14: 实际应用 - 迭代器包装器
///
/// 使用生命周期实现自定义迭代器
pub fn practical_iterator() {
    println!("\n=== 示例 14: 实际应用 - 迭代器包装器 ===");

    // 自定义迭代器，跳过空字符串
    struct NonEmptyIterator<'a> {
        items: &'a [&'a str],
        index: usize,
    }

    impl<'a> NonEmptyIterator<'a> {
        fn new(items: &'a [&'a str]) -> Self {
            NonEmptyIterator { items, index: 0 }
        }
    }

    impl<'a> Iterator for NonEmptyIterator<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            while self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;

                if !item.is_empty() {
                    return Some(item);
                }
            }
            None
        }
    }

    let items = vec!["hello", "", "world", "", "rust", ""];
    let iter = NonEmptyIterator::new(&items);

    println!("原始数据: {:?}", items);
    print!("非空元素: ");
    for item in iter {
        print!("{} ", item);
    }
    println!();

    println!("\n自定义迭代器:");
    println!("  - 使用生命周期引用原始数据");
    println!("  - 零拷贝迭代");
    println!("  - 类型安全");
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Rust 生命周期教学代码                ║");
    println!("╚════════════════════════════════════════╝");

    lifetime_problem();
    lifetime_syntax();
    lifetime_in_functions();
    lifetime_constraints();
    lifetime_in_structs();
    lifetime_elision();
    static_lifetime();
    lifetime_with_generics();
    multiple_lifetimes();
    lifetime_in_methods();
    lifetime_subtyping();
    practical_parser();
    practical_config();
    practical_iterator();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  生命周期是 Rust 内存安全的关键！    ║");
    println!("╚════════════════════════════════════════╝\n");
}

