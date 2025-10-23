//! Rust 集合类型教学代码
//! 
//! 本模块包含 Rust 标准库中常用集合类型的教学示例

use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet};

/// 示例 1: Vector 基础
pub fn vector_basics() {
    println!("\n=== 示例 1: Vector 基础 ===");
    
    // 创建空 Vec
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1: {:?}", v1);
    
    // 使用 vec! 宏创建
    let v2 = vec![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);
    
    // 访问元素
    let third = &v2[2];
    println!("第三个元素: {}", third);
    
    // 使用 get 方法（返回 Option）
    match v2.get(2) {
        Some(third) => println!("第三个元素: {}", third),
        None => println!("没有第三个元素"),
    }
    
    println!("Vector 是可增长的数组，存储在堆上");
}

/// 示例 2: Vector 的常用操作
pub fn vector_operations() {
    println!("\n=== 示例 2: Vector 的常用操作 ===");
    
    let mut v = vec![1, 2, 3, 4, 5];
    
    // 添加元素
    v.push(6);
    println!("push 后: {:?}", v);
    
    // 删除最后一个元素
    let last = v.pop();
    println!("pop 返回: {:?}, 剩余: {:?}", last, v);
    
    // 插入元素
    v.insert(0, 0);
    println!("insert 后: {:?}", v);
    
    // 删除指定位置元素
    let removed = v.remove(0);
    println!("remove 返回: {}, 剩余: {:?}", removed, v);
    
    // 长度和容量
    println!("长度: {}, 容量: {}", v.len(), v.capacity());
    
    // 清空
    v.clear();
    println!("clear 后: {:?}, 长度: {}", v, v.len());
}

/// 示例 3: 遍历 Vector
pub fn vector_iteration() {
    println!("\n=== 示例 3: 遍历 Vector ===");
    
    let v = vec![10, 20, 30, 40, 50];
    
    // 不可变引用遍历
    print!("不可变遍历: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();
    
    // 可变引用遍历
    let mut v2 = vec![1, 2, 3, 4, 5];
    for i in &mut v2 {
        *i *= 2;
    }
    println!("可变遍历后: {:?}", v2);
    
    // 获取所有权遍历
    let v3 = vec![1, 2, 3];
    print!("获取所有权遍历: ");
    for i in v3 {
        print!("{} ", i);
    }
    println!();

    // 带索引遍历
    let v4 = vec!["a", "b", "c"];
    for (index, value) in v4.iter().enumerate() {
        println!("索引 {}: {}", index, value);
    }
}

/// 示例 4: Vector 存储不同类型
pub fn vector_different_types() {
    println!("\n=== 示例 4: Vector 存储不同类型 ===");
    
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("整数: {}", i),
            SpreadsheetCell::Float(f) => println!("浮点数: {}", f),
            SpreadsheetCell::Text(s) => println!("文本: {}", s),
        }
    }
    
    println!("使用枚举可以在 Vector 中存储不同类型的值");
}

/// 示例 5: String 基础
pub fn string_basics() {
    println!("\n=== 示例 5: String 基础 ===");
    
    // 创建空字符串
    let mut s1 = String::new();
    s1.push_str("hello");
    println!("s1: {}", s1);
    
    // 从字符串字面量创建
    let s2 = "initial contents".to_string();
    println!("s2: {}", s2);
    
    // 使用 String::from
    let s3 = String::from("hello");
    println!("s3: {}", s3);
    
    // String 是 UTF-8 编码
    let hello = String::from("你好");
    println!("中文: {}", hello);
    
    let hello = String::from("مرحبا");
    println!("阿拉伯语: {}", hello);
    
    println!("String 是可增长的 UTF-8 编码字符串");
}

/// 示例 6: String 的操作
pub fn string_operations() {
    println!("\n=== 示例 6: String 的操作 ===");
    
    let mut s = String::from("foo");
    
    // 追加字符串切片
    s.push_str("bar");
    println!("push_str 后: {}", s);
    
    // 追加单个字符
    s.push('!');
    println!("push 后: {}", s);
    
    // 使用 + 运算符
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("+ 运算符: {}", s3);
    
    // 使用 format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: {}", s);
    
    // 替换
    let s = String::from("I like cats");
    let new_s = s.replace("cats", "dogs");
    println!("replace: {}", new_s);
}

/// 示例 7: String 和 &str
pub fn string_vs_str() {
    println!("\n=== 示例 7: String 和 &str ===");
    
    // String: 可变的、拥有所有权的字符串
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("String: {}", s);
    
    // &str: 字符串切片，不可变引用
    let slice: &str = &s[0..5];
    println!("&str 切片: {}", slice);
    
    // 字符串字面量是 &str 类型
    let literal: &str = "hello";
    println!("字面量: {}", literal);
    
    // String 可以转换为 &str
    let s = String::from("hello");
    let slice: &str = &s;
    println!("String -> &str: {}", slice);
    
    // &str 可以转换为 String
    let slice = "hello";
    let s = slice.to_string();
    println!("&str -> String: {}", s);
    
    println!("String 拥有数据，&str 是对数据的引用");
}

/// 示例 8: 字符串索引和遍历
pub fn string_indexing() {
    println!("\n=== 示例 8: 字符串索引和遍历 ===");
    
    let s = String::from("hello");
    
    // Rust 不支持直接索引字符串
    // let h = s[0]; // 编译错误
    
    // 使用切片（需要知道字节边界）
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("切片: {}", s);
    
    // 遍历字符
    print!("遍历字符: ");
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();
    
    // 遍历字节
    print!("遍历字节: ");
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!();
    
    println!("字符串是 UTF-8 编码，不能简单地按索引访问");
}

/// 示例 9: HashMap 基础
pub fn hashmap_basics() {
    println!("\n=== 示例 9: HashMap 基础 ===");
    
    // 创建空 HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    
    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("scores: {:?}", scores);
    
    // 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{} 队得分: {}", team_name, s),
        None => println!("队伍不存在"),
    }
    
    // 使用 copied 和 unwrap_or
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("得分: {}", score);
    
    println!("HashMap 存储键值对，键必须是相同类型，值也必须是相同类型");
}

/// 示例 10: HashMap 的操作
pub fn hashmap_operations() {
    println!("\n=== 示例 10: HashMap 的操作 ===");
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // 遍历
    println!("遍历 HashMap:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // 覆盖值
    scores.insert(String::from("Blue"), 25);
    println!("覆盖后: {:?}", scores);
    
    // 只在键不存在时插入
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("entry 后: {:?}", scores);
    
    // 根据旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", map);
}

/// 示例 11: HashMap 的所有权
pub fn hashmap_ownership() {
    println!("\n=== 示例 11: HashMap 的所有权 ===");

    // 实现了 Copy trait 的类型，值会被复制
    let mut map = HashMap::new();
    let key = 1;
    let value = 10;
    map.insert(key, value);
    println!("key: {}, value: {}", key, value);

    // 没有实现 Copy trait 的类型，所有权会被移动
    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map.insert(field_name, field_value);

    // field_name 和 field_value 的所有权已经移动到 map
    // println!("{}", field_name); // 编译错误

    println!("map: {:?}", map);
    println!("HashMap 会获取没有实现 Copy trait 的值的所有权");
}

/// 示例 12: BTreeMap 基础
pub fn btreemap_basics() {
    println!("\n=== 示例 12: BTreeMap 基础 ===");

    let mut map = BTreeMap::new();
    map.insert(3, "c");
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(5, "e");
    map.insert(4, "d");

    println!("BTreeMap（按键排序）:");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 获取第一个和最后一个
    if let Some((first_key, first_value)) = map.first_key_value() {
        println!("第一个: {} -> {}", first_key, first_value);
    }

    if let Some((last_key, last_value)) = map.last_key_value() {
        println!("最后一个: {} -> {}", last_key, last_value);
    }

    println!("BTreeMap 按键排序，HashMap 无序但更快");
}

/// 示例 13: HashSet 基础
pub fn hashset_basics() {
    println!("\n=== 示例 13: HashSet 基础 ===");

    let mut books = HashSet::new();

    // 插入元素
    books.insert("The Rust Programming Language");
    books.insert("Programming Rust");
    books.insert("Rust in Action");

    // 重复插入会被忽略
    if !books.insert("The Rust Programming Language") {
        println!("已经有这本书了");
    }

    println!("书籍数量: {}", books.len());

    // 检查是否包含
    let book = "Programming Rust";
    if books.contains(book) {
        println!("包含: {}", book);
    }

    // 删除元素
    books.remove("Rust in Action");
    println!("删除后数量: {}", books.len());

    println!("HashSet 存储唯一值，没有重复");
}

/// 示例 14: HashSet 的集合操作
pub fn hashset_operations() {
    println!("\n=== 示例 14: HashSet 的集合操作 ===");

    let set1: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<_> = [4, 5, 6, 7, 8].iter().cloned().collect();

    // 并集
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("并集: {:?}", union);

    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("交集: {:?}", intersection);

    // 差集
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("差集 (set1 - set2): {:?}", difference);

    // 对称差集
    let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    println!("对称差集: {:?}", symmetric_difference);

    // 子集和超集
    let set3: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    println!("set3 是 set1 的子集: {}", set3.is_subset(&set1));
    println!("set1 是 set3 的超集: {}", set1.is_superset(&set3));
}

/// 示例 15: BTreeSet 基础
pub fn btreeset_basics() {
    println!("\n=== 示例 15: BTreeSet 基础 ===");

    let mut set = BTreeSet::new();
    set.insert(5);
    set.insert(2);
    set.insert(8);
    set.insert(1);
    set.insert(3);

    println!("BTreeSet（排序）:");
    for value in &set {
        print!("{} ", value);
    }
    println!();

    // 范围查询
    println!("范围 2..=5:");
    for value in set.range(2..=5) {
        print!("{} ", value);
    }
    println!();

    println!("BTreeSet 保持元素排序，HashSet 无序但更快");
}

/// 示例 16: 实际应用 - 学生成绩管理
pub fn practical_student_scores() {
    println!("\n=== 示例 16: 实际应用 - 学生成绩管理 ===");

    let mut scores: HashMap<String, Vec<i32>> = HashMap::new();

    // 添加成绩
    scores.entry(String::from("Alice")).or_insert(Vec::new()).push(85);
    scores.entry(String::from("Alice")).or_insert(Vec::new()).push(90);
    scores.entry(String::from("Bob")).or_insert(Vec::new()).push(78);
    scores.entry(String::from("Bob")).or_insert(Vec::new()).push(82);

    // 计算平均分
    for (name, score_list) in &scores {
        let sum: i32 = score_list.iter().sum();
        let avg = sum as f64 / score_list.len() as f64;
        println!("{}: 成绩 {:?}, 平均分 {:.2}", name, score_list, avg);
    }
}

/// 示例 17: 实际应用 - 去重和排序
pub fn practical_dedup_and_sort() {
    println!("\n=== 示例 17: 实际应用 - 去重和排序 ===");

    let numbers = vec![4, 2, 7, 2, 9, 4, 1, 7, 3];
    println!("原始数据: {:?}", numbers);

    // 使用 HashSet 去重
    let unique: HashSet<_> = numbers.iter().cloned().collect();
    println!("去重后: {:?}", unique);

    // 使用 BTreeSet 去重并排序
    let sorted_unique: BTreeSet<_> = numbers.iter().cloned().collect();
    println!("去重并排序: {:?}", sorted_unique);

    // 转回 Vec
    let result: Vec<_> = sorted_unique.iter().cloned().collect();
    println!("转回 Vec: {:?}", result);
}

/// 示例 18: 实际应用 - 文本分析
pub fn practical_text_analysis() {
    println!("\n=== 示例 18: 实际应用 - 文本分析 ===");

    let text = "the quick brown fox jumps over the lazy dog the fox is quick";

    // 单词频率统计
    let mut word_count: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }

    println!("单词频率:");
    let mut counts: Vec<_> = word_count.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1));
    for (word, count) in counts {
        println!("  {}: {}", word, count);
    }

    // 唯一单词
    let unique_words: HashSet<_> = text.split_whitespace().collect();
    println!("唯一单词数: {}", unique_words.len());
    println!("总单词数: {}", text.split_whitespace().count());
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 集合类型教学代码               ║");
    println!("╚════════════════════════════════════════╝");

    vector_basics();
    vector_operations();
    vector_iteration();
    vector_different_types();
    string_basics();
    string_operations();
    string_vs_str();
    string_indexing();
    hashmap_basics();
    hashmap_operations();
    hashmap_ownership();
    btreemap_basics();
    hashset_basics();
    hashset_operations();
    btreeset_basics();
    practical_student_scores();
    practical_dedup_and_sort();
    practical_text_analysis();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  集合类型是 Rust 程序的基础工具！   ║");
    println!("╚════════════════════════════════════════╝");
}


