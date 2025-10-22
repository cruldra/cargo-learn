// Rust 数据类型教学代码
// 主题：标量类型、复合类型

/// 示例 1: 整数类型
/// Rust 提供了多种整数类型：i8, i16, i32, i64, i128, isize 和 u8, u16, u32, u64, u128, usize
pub fn integer_types() {
    println!("\n=== 示例 1: 整数类型 ===");
    
    // 有符号整数（可以表示正数和负数）
    let _signed_8bit: i8 = -128;
    let _signed_16bit: i16 = -32768;
    let _signed_32bit: i32 = -2147483648;
    let _signed_64bit: i64 = -9223372036854775808;
    
    println!("i8 范围: {} 到 {}", i8::MIN, i8::MAX);
    println!("i16 范围: {} 到 {}", i16::MIN, i16::MAX);
    println!("i32 范围: {} 到 {}", i32::MIN, i32::MAX);
    println!("i64 范围: {} 到 {}", i64::MIN, i64::MAX);
    
    // 无符号整数（只能表示非负数）
    let _unsigned_8bit: u8 = 255;
    let _unsigned_16bit: u16 = 65535;
    let _unsigned_32bit: u32 = 4294967295;
    
    println!("\nu8 范围: {} 到 {}", u8::MIN, u8::MAX);
    println!("u16 范围: {} 到 {}", u16::MIN, u16::MAX);
    println!("u32 范围: {} 到 {}", u32::MIN, u32::MAX);
    
    // 架构相关的整数类型
    let _arch_signed: isize = -100;
    let _arch_unsigned: usize = 100;
    println!("\nisize 和 usize 的大小取决于系统架构（32位或64位）");
    println!("当前系统 isize 大小: {} 字节", std::mem::size_of::<isize>());
}

/// 示例 2: 整数字面量
/// Rust 支持多种整数字面量表示方式
pub fn integer_literals() {
    println!("\n=== 示例 2: 整数字面量 ===");
    
    let decimal = 98_222;           // 十进制，可以用下划线分隔
    let hex = 0xff;                 // 十六进制
    let octal = 0o77;               // 八进制
    let binary = 0b1111_0000;       // 二进制
    let byte = b'A';                // 字节（仅限 u8）
    
    println!("十进制: {}", decimal);
    println!("十六进制 0xff: {}", hex);
    println!("八进制 0o77: {}", octal);
    println!("二进制 0b1111_0000: {}", binary);
    println!("字节 b'A': {}", byte);
    
    // 类型后缀
    let with_suffix = 42u32;
    println!("\n带类型后缀的字面量: {}", with_suffix);
}

/// 示例 3: 浮点数类型
/// Rust 有两种浮点数类型：f32 和 f64
pub fn floating_point_types() {
    println!("\n=== 示例 3: 浮点数类型 ===");
    
    let f32_num: f32 = 3.14159265;
    let f64_num: f64 = 3.14159265358979323846;
    
    println!("f32 (单精度): {}", f32_num);
    println!("f64 (双精度): {}", f64_num);
    
    // 默认类型是 f64
    let _default_float = 2.0;
    println!("\n默认浮点数类型是 f64");
    
    // 科学计数法
    let scientific = 1.23e-4;
    println!("科学计数法: {}", scientific);
    
    // 特殊值
    println!("\n特殊浮点数值:");
    println!("正无穷: {}", f64::INFINITY);
    println!("负无穷: {}", f64::NEG_INFINITY);
    println!("NaN (非数字): {}", f64::NAN);
}

/// 示例 4: 数值运算
/// 基本的数学运算
pub fn numeric_operations() {
    println!("\n=== 示例 4: 数值运算 ===");
    
    // 加法
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);
    
    // 减法
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);
    
    // 乘法
    let product = 4 * 30;
    println!("4 * 30 = {}", product);
    
    // 除法
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);
    
    let truncated = 5 / 3; // 整数除法会截断
    println!("5 / 3 = {} (整数除法)", truncated);
    
    // 取余
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);
}

/// 示例 5: 布尔类型
/// 布尔类型只有两个值：true 和 false
pub fn boolean_type() {
    println!("\n=== 示例 5: 布尔类型 ===");
    
    let t = true;
    let f: bool = false;
    
    println!("true: {}", t);
    println!("false: {}", f);
    
    // 布尔运算
    println!("\n布尔运算:");
    println!("true && false = {}", true && false);
    println!("true || false = {}", true || false);
    println!("!true = {}", !true);
    
    // 比较运算返回布尔值
    println!("\n比较运算:");
    println!("5 > 3 = {}", 5 > 3);
    println!("5 < 3 = {}", 5 < 3);
    println!("5 == 5 = {}", 5 == 5);
    println!("5 != 3 = {}", 5 != 3);
}

/// 示例 6: 字符类型
/// char 类型表示单个 Unicode 字符
pub fn character_type() {
    println!("\n=== 示例 6: 字符类型 ===");
    
    let c = 'z';
    let z: char = 'ℤ';
    let heart = '❤';
    let chinese = '中';
    
    println!("英文字符: {}", c);
    println!("数学符号: {}", z);
    println!("表情符号: {}", heart);
    println!("中文字符: {}", chinese);
    
    println!("\nchar 类型占用 4 字节，可以表示任何 Unicode 字符");
    println!("char 大小: {} 字节", std::mem::size_of::<char>());
}

/// 示例 7: 元组类型
/// 元组可以将多个不同类型的值组合成一个复合类型
pub fn tuple_type() {
    println!("\n=== 示例 7: 元组类型 ===");
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("元组: {:?}", tup);
    
    // 解构元组
    let (x, y, z) = tup;
    println!("解构后: x={}, y={}, z={}", x, y, z);
    
    // 使用索引访问元组元素
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("通过索引访问: {}, {}, {}", five_hundred, six_point_four, one);
    
    // 单元类型（空元组）
    let unit = ();
    println!("\n单元类型（空元组）: {:?}", unit);
}

/// 示例 8: 数组类型
/// 数组中的每个元素必须是相同类型，且长度固定
pub fn array_type() {
    println!("\n=== 示例 8: 数组类型 ===");
    
    let arr = [1, 2, 3, 4, 5];
    println!("数组: {:?}", arr);
    
    // 显式类型标注
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("带类型标注的数组: {:?}", arr2);
    
    // 使用相同值初始化数组
    let arr3 = [3; 5]; // 等同于 [3, 3, 3, 3, 3]
    println!("重复值数组: {:?}", arr3);
    
    // 访问数组元素
    let first = arr[0];
    let second = arr[1];
    println!("第一个元素: {}", first);
    println!("第二个元素: {}", second);
    
    // 数组长度
    println!("数组长度: {}", arr.len());
}

/// 示例 9: 切片类型
/// 切片是对数组或其他集合的引用
pub fn slice_type() {
    println!("\n=== 示例 9: 切片类型 ===");
    
    let arr = [1, 2, 3, 4, 5];
    
    // 创建切片
    let slice = &arr[1..4]; // 包含索引 1, 2, 3
    println!("原数组: {:?}", arr);
    println!("切片 [1..4]: {:?}", slice);
    
    let slice2 = &arr[..3]; // 从开始到索引 2
    println!("切片 [..3]: {:?}", slice2);
    
    let slice3 = &arr[2..]; // 从索引 2 到结束
    println!("切片 [2..]: {:?}", slice3);
    
    let slice4 = &arr[..]; // 整个数组
    println!("切片 [..]: {:?}", slice4);
}

/// 示例 10: 字符串类型
/// Rust 有两种主要的字符串类型：&str 和 String
pub fn string_types() {
    println!("\n=== 示例 10: 字符串类型 ===");
    
    // 字符串字面量（&str）
    let str_literal: &str = "Hello, Rust!";
    println!("字符串字面量: {}", str_literal);
    
    // String 类型（可增长的字符串）
    let mut string = String::from("Hello");
    println!("String: {}", string);
    
    string.push_str(", Rust!");
    println!("追加后: {}", string);
    
    // 字符串切片
    let slice = &string[0..5];
    println!("字符串切片: {}", slice);
    
    println!("\n&str 是不可变引用，String 是可增长的堆分配字符串");
}

/// 示例 11: 类型转换
/// Rust 不会自动进行类型转换，需要显式转换
pub fn type_conversion() {
    println!("\n=== 示例 11: 类型转换 ===");
    
    // 使用 as 进行类型转换
    let integer = 42;
    let float = integer as f64;
    println!("整数 {} 转换为浮点数 {}", integer, float);
    
    let large = 300u16;
    let small = large as u8; // 可能会截断
    println!("u16 {} 转换为 u8 {} (注意溢出)", large, small);
    
    // 字符串转数字
    let num_str = "42";
    let num: i32 = num_str.parse().unwrap();
    println!("字符串 '{}' 转换为数字 {}", num_str, num);
    
    // 数字转字符串
    let num = 42;
    let num_string = num.to_string();
    println!("数字 {} 转换为字符串 '{}'", num, num_string);
}

/// 示例 12: 类型别名
/// 使用 type 关键字创建类型别名
pub fn type_aliases() {
    println!("\n=== 示例 12: 类型别名 ===");
    
    type Kilometers = i32;
    type Point = (i32, i32);
    
    let distance: Kilometers = 100;
    println!("距离: {} 公里", distance);
    
    let origin: Point = (0, 0);
    let destination: Point = (10, 20);
    println!("起点: {:?}", origin);
    println!("终点: {:?}", destination);
    
    println!("\n类型别名可以让代码更具可读性");
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 数据类型教学代码               ║");
    println!("╚════════════════════════════════════════╝");
    
    integer_types();
    integer_literals();
    floating_point_types();
    numeric_operations();
    boolean_type();
    character_type();
    tuple_type();
    array_type();
    slice_type();
    string_types();
    type_conversion();
    type_aliases();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  所有示例运行完毕！                  ║");
    println!("╚════════════════════════════════════════╝");
}

