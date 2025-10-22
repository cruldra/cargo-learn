// Rust 引用与借用教学代码
// 主题：引用、借用、可变引用、不可变引用、借用规则

/// 示例 1: 引用基础
/// 引用允许你使用值但不获取其所有权
pub fn basic_references() {
    println!("\n=== 示例 1: 引用基础 ===");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // &s1 创建一个引用
    
    println!("字符串 '{}' 的长度是 {}", s1, len);
    println!("s1 仍然有效，因为我们只是借用了它");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 离开作用域，但因为它不拥有引用的值，所以什么也不会发生

/// 示例 2: 引用与所有权的对比
/// 理解引用和所有权转移的区别
pub fn references_vs_ownership() {
    println!("\n=== 示例 2: 引用与所有权的对比 ===");
    
    let s1 = String::from("hello");
    
    // 使用引用，不转移所有权
    let len1 = get_length_ref(&s1);
    println!("使用引用: s1 = {}, len = {}", s1, len1);
    
    // 转移所有权
    let s2 = String::from("world");
    let (s2_back, len2) = get_length_own(s2);
    println!("转移所有权: s2 = {}, len = {}", s2_back, len2);
}

fn get_length_ref(s: &String) -> usize {
    s.len()
}

fn get_length_own(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

/// 示例 3: 不可变引用
/// 可以同时存在多个不可变引用
pub fn immutable_references() {
    println!("\n=== 示例 3: 不可变引用 ===");
    
    let s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("r1 = {}, r2 = {}, r3 = {}", r1, r2, r3);
    println!("可以同时存在多个不可变引用");
}

/// 示例 4: 可变引用
/// 使用可变引用修改值
pub fn mutable_references() {
    println!("\n=== 示例 4: 可变引用 ===");
    
    let mut s = String::from("hello");
    
    change(&mut s);
    
    println!("修改后: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

/// 示例 5: 可变引用的限制
/// 在特定作用域中，只能有一个可变引用
pub fn mutable_reference_restrictions() {
    println!("\n=== 示例 5: 可变引用的限制 ===");
    
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    r1.push_str(" world");
    println!("r1 = {}", r1);
    
    // let r2 = &mut s;  // 错误！不能同时有两个可变引用
    // println!("{}, {}", r1, r2);
    
    println!("在同一作用域中，只能有一个可变引用");
}

/// 示例 6: 可变引用与不可变引用不能共存
/// 不能在拥有不可变引用的同时拥有可变引用
pub fn mixed_references() {
    println!("\n=== 示例 6: 可变引用与不可变引用不能共存 ===");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);
    // r1 和 r2 的作用域在这里结束（最后一次使用）

    let r3 = &mut s;  // 可以！因为 r1 和 r2 已经不再使用
    r3.push_str(" world");
    println!("r3 = {}", r3);

    println!("引用的作用域从声明开始，到最后一次使用结束");
}

/// 示例 7: 借用规则总结
/// Rust 的借用规则
pub fn borrowing_rules() {
    println!("\n=== 示例 7: 借用规则总结 ===");
    
    println!("借用规则：");
    println!("1. 在任意给定时间，要么只能有一个可变引用");
    println!("2. 要么只能有多个不可变引用");
    println!("3. 引用必须总是有效的");
    
    let mut s = String::from("hello");
    
    // 规则 1 和 2：多个不可变引用
    {
        let r1 = &s;
        let r2 = &s;
        println!("不可变引用: {}, {}", r1, r2);
    }
    
    // 规则 1 和 2：一个可变引用
    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("可变引用: {}", r1);
    }
}

/// 示例 8: 悬垂引用
/// Rust 编译器防止悬垂引用
pub fn dangling_references() {
    println!("\n=== 示例 8: 悬垂引用 ===");
    
    // let reference_to_nothing = dangle();  // 错误！
    let s = no_dangle();
    println!("正确的做法: {}", s);
}

// fn dangle() -> &String {  // 错误！返回悬垂引用
//     let s = String::from("hello");
//     &s  // s 离开作用域被丢弃，引用指向无效内存
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 直接返回 String，转移所有权
}

/// 示例 9: 引用作为函数参数
/// 使用引用避免所有权转移
pub fn references_as_parameters() {
    println!("\n=== 示例 9: 引用作为函数参数 ===");
    
    let s = String::from("hello world");
    
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    println!("原字符串仍然有效: {}", s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/// 示例 10: 可变引用修改数据
/// 通过可变引用修改原始数据
pub fn modify_through_mutable_reference() {
    println!("\n=== 示例 10: 可变引用修改数据 ===");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("修改前: {:?}", numbers);
    
    double_values(&mut numbers);
    println!("修改后: {:?}", numbers);
}

fn double_values(nums: &mut Vec<i32>) {
    for num in nums.iter_mut() {
        *num *= 2;
    }
}

/// 示例 11: 引用的作用域
/// 理解引用的作用域（NLL - Non-Lexical Lifetimes）
pub fn reference_scope() {
    println!("\n=== 示例 11: 引用的作用域 ===");
    
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 和 r2 的作用域在这里结束（最后一次使用）
    
    let r3 = &mut s;  // 可以！
    r3.push_str(" world");
    println!("{}", r3);
}

/// 示例 12: 多个可变引用（不同作用域）
/// 在不同作用域中可以有多个可变引用
pub fn multiple_mutable_references() {
    println!("\n=== 示例 12: 多个可变引用（不同作用域） ===");
    
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("第一个作用域: {}", r1);
    }  // r1 离开作用域
    
    {
        let r2 = &mut s;
        r2.push_str("!");
        println!("第二个作用域: {}", r2);
    }  // r2 离开作用域
    
    println!("最终结果: {}", s);
}

/// 示例 13: 引用与切片
/// 切片是一种特殊的引用
pub fn references_and_slices() {
    println!("\n=== 示例 13: 引用与切片 ===");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    
    println!("切片 1: {}", hello);
    println!("切片 2: {}", world);
    println!("原字符串: {}", s);
    
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("数组切片: {:?}", slice);
}

/// 示例 14: 实际应用 - 查找和替换
pub fn practical_find_and_replace() {
    println!("\n=== 示例 14: 实际应用 - 查找和替换 ===");
    
    let mut text = String::from("Hello Rust");
    println!("原文本: {}", text);
    
    if contains_word(&text, "Rust") {
        replace_word(&mut text, "Rust", "World");
    }
    
    println!("修改后: {}", text);
}

fn contains_word(text: &String, word: &str) -> bool {
    text.contains(word)
}

fn replace_word(text: &mut String, from: &str, to: &str) {
    *text = text.replace(from, to);
}

/// 示例 15: 实际应用 - 数据验证
pub fn practical_data_validation() {
    println!("\n=== 示例 15: 实际应用 - 数据验证 ===");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    if validate_data(&data) {
        println!("数据验证通过");
        process_data(&mut data);
        println!("处理后的数据: {:?}", data);
    }
}

fn validate_data(data: &Vec<i32>) -> bool {
    !data.is_empty() && data.len() <= 10
}

fn process_data(data: &mut Vec<i32>) {
    data.iter_mut().for_each(|x| *x *= 2);
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 引用与借用教学代码             ║");
    println!("╚════════════════════════════════════════╝");
    
    basic_references();
    references_vs_ownership();
    immutable_references();
    mutable_references();
    mutable_reference_restrictions();
    mixed_references();
    borrowing_rules();
    dangling_references();
    references_as_parameters();
    modify_through_mutable_reference();
    reference_scope();
    multiple_mutable_references();
    references_and_slices();
    practical_find_and_replace();
    practical_data_validation();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  借用规则保证了内存安全！           ║");
    println!("║  编译时检查，运行时零开销           ║");
    println!("╚════════════════════════════════════════╝");
}

