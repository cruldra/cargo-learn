// ============================================
// 导入标准库和第三方库
// ============================================

// 导入文件操作相关的类型
use std::fs::File;

// 导入 IO 相关的类型
// io 是 std::io 模块本身，用于错误类型
// Read 是一个 trait，提供读取数据的方法
use std::io::{self, Read};

// 导入整数解析错误类型
// 当字符串无法解析为整数时会返回这个错误
use std::num::ParseIntError;

// 导入 thiserror 库的 Error 派生宏
// thiserror 用于简化自定义错误类型的定义
use thiserror::Error;

// 导入 anyhow 库的上下文功能和结果类型
// Context trait 提供 .context() 方法来添加错误上下文
// Result 是 anyhow 的结果类型，重命名为 AnyhowResult 避免与标准库的 Result 冲突
use anyhow::{Context, Result as AnyhowResult};

/// 示例 1: panic! 宏基础
///
/// panic! 用于不可恢复的错误，会立即终止程序
/// 这与其他语言的异常（throw）不同，panic 表示程序遇到了严重的 bug
pub fn panic_basics() {
    println!("\n=== 示例 1: panic! 宏基础 ===");

    // panic! 会导致程序立即崩溃并打印错误信息
    // 程序会展开栈（unwind stack），清理资源，然后退出
    // panic!("程序崩溃了！");

    // 带格式化参数的 panic
    // 可以像 println! 一样使用格式化字符串
    let x = 5;
    // panic!("x 的值是 {}", x);

    // 数组越界会自动触发 panic
    // Rust 在运行时检查数组边界，防止内存安全问题
    let v = vec![1, 2, 3];
    // let element = v[99]; // 这会 panic: index out of bounds

    println!("panic! 会立即终止程序");
    println!("通常用于不可恢复的错误");
    println!("示例代码已注释，避免程序崩溃");
}

/// 示例 2: unwrap 和 expect
///
/// unwrap 和 expect 是快速提取 Result 中值的方法
/// 但如果遇到错误会 panic，所以只适合原型开发或确定不会出错的情况
pub fn unwrap_and_expect() {
    println!("\n=== 示例 2: unwrap 和 expect ===");

    // unwrap: 如果是 Ok(value) 返回 value，如果是 Err 则 panic
    // 这里明确指定类型：Result<成功类型, 错误类型>
    let result: Result<i32, &str> = Ok(42);
    let value = result.unwrap(); // 因为是 Ok(42)，所以返回 42
    println!("unwrap 成功: {}", value);

    // expect: 和 unwrap 类似，但可以自定义 panic 时的错误信息
    // 这样在出错时能提供更多上下文信息
    let result: Result<i32, &str> = Ok(100);
    let value = result.expect("应该是 Ok"); // 如果是 Err 会显示这个消息
    println!("expect 成功: {}", value);

    // 下面这些会导致 panic（已注释避免程序崩溃）
    // let result: Result<i32, &str> = Err("错误");
    // result.unwrap(); // panic: called `Result::unwrap()` on an `Err` value: "错误"
    // result.expect("自定义错误信息"); // panic: 自定义错误信息: "错误"

    println!("unwrap 和 expect 适合原型开发");
    println!("生产代码应该正确处理错误");
}

/// 示例 3: Result 类型基础
///
/// Result<T, E> 是 Rust 处理可恢复错误的核心类型
/// T 是成功时的值类型，E 是错误时的错误类型
/// Result 是一个枚举：enum Result<T, E> { Ok(T), Err(E) }
pub fn result_basics() {
    println!("\n=== 示例 3: Result 类型基础 ===");

    // 定义一个可能失败的函数
    // 返回类型 Result<f64, String> 表示：
    // - 成功时返回 Ok(f64)
    // - 失败时返回 Err(String)
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            // 返回错误：Err 包含错误信息
            Err(String::from("除数不能为零"))
        } else {
            // 返回成功：Ok 包含计算结果
            Ok(a / b)
        }
    }

    // 使用 match 表达式处理 Result
    // match 强制你处理所有可能的情况（Ok 和 Err）
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result), // 成功分支
        Err(e) => println!("错误: {}", e),              // 错误分支
    }

    // 处理除以零的情况
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("错误: {}", e), // 这个分支会被执行
    }

    println!("Result<T, E> 用于可恢复的错误");
}

/// 示例 4: Result 的常用方法
///
/// Result 提供了很多方便的方法来处理成功和错误的情况
/// 这些方法可以让你避免写大量的 match 表达式
pub fn result_methods() {
    println!("\n=== 示例 4: Result 的常用方法 ===");

    // 创建两个 Result 用于演示
    let ok: Result<i32, &str> = Ok(42);   // 成功的结果
    let err: Result<i32, &str> = Err("错误"); // 失败的结果

    // is_ok() 和 is_err() 用于检查 Result 的状态
    // 返回 bool 值，不消耗 Result
    println!("ok.is_ok(): {}", ok.is_ok());     // true
    println!("err.is_err(): {}", err.is_err()); // true

    // unwrap_or: 如果是 Ok(value) 返回 value，如果是 Err 返回提供的默认值
    // 这是一个安全的方法，不会 panic
    println!("err.unwrap_or(0): {}", err.unwrap_or(0)); // 返回 0

    // unwrap_or_else: 如果是 Err，调用闭包生成默认值
    // 闭包接收错误值作为参数
    // 这比 unwrap_or 更灵活，因为可以根据错误内容决定返回什么
    let result = err.unwrap_or_else(|e| {
        println!("  错误: {}", e); // e 是 "错误"
        0  // 返回默认值
    });
    println!("err.unwrap_or_else(...): {}", result);

    // map: 如果是 Ok(value)，对 value 应用函数，返回 Ok(新值)
    // 如果是 Err，直接返回 Err，不做任何处理
    // 这类似于 Option 的 map
    let doubled = ok.map(|x| x * 2); // Ok(42) -> Ok(84)
    println!("ok.map(|x| x * 2): {:?}", doubled);

    // map_err: 和 map 相反，只转换 Err 中的值
    // 如果是 Ok，直接返回 Ok
    // 如果是 Err(e)，对 e 应用函数，返回 Err(新错误)
    let mapped_err = err.map_err(|e| format!("映射后的错误: {}", e));
    println!("err.map_err(...): {:?}", mapped_err);

    // and_then: 链式调用，用于连续的可能失败的操作
    // 如果是 Ok(value)，调用闭包，闭包必须返回一个 Result
    // 如果是 Err，直接返回 Err
    // 这类似于其他语言的 flatMap
    let result = ok.and_then(|x| Ok(x + 10)); // Ok(42) -> Ok(52)
    println!("ok.and_then(|x| Ok(x + 10)): {:?}", result);
}

/// 示例 5: ? 操作符基础
///
/// ? 操作符是 Rust 错误处理的语法糖
/// 它可以大大简化错误传播的代码
///
/// ? 的工作原理：
/// - 如果 Result 是 Ok(value)，返回 value
/// - 如果 Result 是 Err(e)，立即从当前函数返回 Err(e)
pub fn question_mark_basics() -> Result<(), String> {
    println!("\n=== 示例 5: ? 操作符基础 ===");

    // 辅助函数：将字符串解析为数字
    // map_err 将 ParseIntError 转换为 String
    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse::<i32>()  // 返回 Result<i32, ParseIntError>
            .map_err(|e| e.to_string())  // 转换为 Result<i32, String>
    }

    // 方式 1: 不使用 ? 操作符（传统方式）
    // 需要手动用 match 检查每个 Result，代码冗长
    fn add_without_question_mark(a: &str, b: &str) -> Result<i32, String> {
        // 解析第一个数字
        let num1 = match parse_number(a) {
            Ok(n) => n,           // 如果成功，提取值
            Err(e) => return Err(e),  // 如果失败，立即返回错误
        };
        // 解析第二个数字
        let num2 = match parse_number(b) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        // 两个都成功，返回和
        Ok(num1 + num2)
    }

    // 方式 2: 使用 ? 操作符（推荐方式）
    // ? 会自动做上面 match 做的事情，代码简洁很多
    fn add_with_question_mark(a: &str, b: &str) -> Result<i32, String> {
        let num1 = parse_number(a)?;  // 如果失败，自动返回 Err
        let num2 = parse_number(b)?;  // 如果失败，自动返回 Err
        Ok(num1 + num2)  // 两个都成功，返回和
    }

    // 测试两种方式
    println!("不使用 ?: {:?}", add_without_question_mark("10", "20")); // Ok(30)
    println!("使用 ?: {:?}", add_with_question_mark("10", "20"));      // Ok(30)
    println!("错误传播: {:?}", add_with_question_mark("10", "abc"));   // Err(...)

    println!("? 操作符自动传播错误");
    Ok(())  // 这个函数也返回 Result，所以需要返回 Ok(())
}

/// 示例 6: ? 操作符的错误转换
///
/// ? 操作符的强大之处在于它可以自动转换错误类型
/// 只要错误类型实现了 From trait，? 就会自动调用转换
///
/// Box<dyn std::error::Error> 是一个"万能"错误类型
/// 可以包含任何实现了 Error trait 的错误
pub fn question_mark_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 6: ? 操作符的错误转换 ===");

    // 这个函数返回 Box<dyn Error>，但内部使用的是 ParseIntError
    // ? 操作符会自动调用 From trait 进行转换
    fn read_number_from_string(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
        // s.parse() 返回 Result<i32, ParseIntError>
        // ? 会自动将 ParseIntError 转换为 Box<dyn Error>
        // 这是因为 ParseIntError 实现了 Error trait
        let num: i32 = s.parse()?;
        Ok(num)
    }

    // 测试成功的情况
    match read_number_from_string("42") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }

    // 测试失败的情况
    match read_number_from_string("abc") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),  // 会打印 ParseIntError 的信息
    }

    println!("? 操作符可以自动转换兼容的错误类型");
    Ok(())
}

/// 示例 7: 自定义错误类型（手动实现）
///
/// 自定义错误类型可以提供更精确的错误信息
/// 需要实现两个 trait：
/// 1. Display - 用于显示错误信息
/// 2. Error - 标记这是一个错误类型
pub fn custom_error_manual() {
    println!("\n=== 示例 7: 自定义错误类型（手动实现） ===");

    use std::fmt;

    // 定义一个数学错误枚举
    // #[derive(Debug)] 自动实现 Debug trait，用于调试输出
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,      // 除以零错误
        NegativeSquareRoot,  // 负数开平方根错误
    }

    // 实现 Display trait，定义错误如何显示给用户
    // 这是必须的，因为 Error trait 要求实现 Display
    impl fmt::Display for MathError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 根据不同的错误类型返回不同的错误信息
            match self {
                MathError::DivisionByZero => write!(f, "除数不能为零"),
                MathError::NegativeSquareRoot => write!(f, "不能对负数开平方根"),
            }
        }
    }

    // 实现 Error trait，标记这是一个错误类型
    // 这样就可以在 Result<T, E> 中使用，也可以转换为 Box<dyn Error>
    impl std::error::Error for MathError {}

    // 除法函数，可能返回 DivisionByZero 错误
    fn divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)  // 返回自定义错误
        } else {
            Ok(a / b)
        }
    }

    // 平方根函数，可能返回 NegativeSquareRoot 错误
    fn sqrt(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)  // 返回自定义错误
        } else {
            Ok(x.sqrt())  // 调用标准库的 sqrt 方法
        }
    }

    // 测试各种情况
    println!("10 / 2 = {:?}", divide(10.0, 2.0));   // Ok(5.0)
    println!("10 / 0 = {:?}", divide(10.0, 0.0));   // Err(DivisionByZero)
    println!("sqrt(4) = {:?}", sqrt(4.0));          // Ok(2.0)
    println!("sqrt(-1) = {:?}", sqrt(-1.0));        // Err(NegativeSquareRoot)

    println!("自定义错误类型提供更好的类型安全");
}

/// 示例 8: 使用 thiserror 简化错误定义
///
/// thiserror 是一个库，可以大大简化自定义错误类型的定义
/// 它通过派生宏自动实现 Display 和 Error trait
///
/// 主要特性：
/// - #[derive(Error)] 自动实现 Error trait
/// - #[error("...")] 定义错误信息
/// - #[from] 自动实现 From trait，用于错误转换
pub fn thiserror_example() {
    println!("\n=== 示例 8: 使用 thiserror 简化错误定义 ===");

    // 使用 thiserror 定义错误类型
    // #[derive(Error, Debug)] 自动实现 Error 和 Debug trait
    #[derive(Error, Debug)]
    enum DataError {
        // #[error("...")] 定义这个错误的显示信息
        // {0} 是占位符，会被元组字段的值替换
        #[error("IO 错误: {0}")]
        Io(#[from] io::Error),  // #[from] 自动实现 From<io::Error> for DataError

        #[error("解析错误: {0}")]
        Parse(#[from] ParseIntError),  // #[from] 自动实现 From<ParseIntError> for DataError

        // 不带 #[from] 的变体，需要手动构造
        #[error("数据验证失败: {0}")]
        Validation(String),

        // 没有数据的变体
        #[error("未找到数据")]
        NotFound,
    }

    // 使用自定义错误类型的函数
    fn process_data(valid: bool) -> Result<i32, DataError> {
        if !valid {
            // 手动返回 Validation 错误
            return Err(DataError::Validation(String::from("数据无效")));
        }

        // "42".parse() 返回 Result<i32, ParseIntError>
        // ? 操作符会自动调用 From<ParseIntError> for DataError
        // 将 ParseIntError 转换为 DataError::Parse
        let num: i32 = "42".parse()?;
        Ok(num)
    }

    // 测试
    println!("有效数据: {:?}", process_data(true));   // Ok(42)
    println!("无效数据: {:?}", process_data(false));  // Err(Validation("数据无效"))

    println!("thiserror 自动实现 Display 和 Error trait");
    println!("#[from] 自动实现错误转换");
}

/// 示例 9: 使用 anyhow 简化错误处理
///
/// anyhow 是一个用于应用程序（不是库）的错误处理库
/// 它提供了一个统一的错误类型 anyhow::Error
/// 可以包装任何实现了 std::error::Error 的错误
///
/// 主要优势：
/// - 不需要定义自己的错误类型
/// - 可以轻松添加上下文信息
/// - 简化错误传播
pub fn anyhow_example() -> AnyhowResult<()> {
    println!("\n=== 示例 9: 使用 anyhow 简化错误处理 ===");

    // 使用 anyhow::Result，可以返回任何错误
    // AnyhowResult<T> 是 Result<T, anyhow::Error> 的别名
    fn read_config() -> AnyhowResult<String> {
        // parse 返回 Result<i32, ParseIntError>
        // ? 会自动将 ParseIntError 转换为 anyhow::Error
        let content = "42".parse::<i32>()?;
        Ok(content.to_string())
    }

    // 另一个使用 anyhow 的函数
    fn process() -> AnyhowResult<()> {
        // .context() 方法添加错误上下文
        // 如果 read_config() 失败，错误信息会包含 "读取配置失败"
        let config = read_config()
            .context("读取配置失败")?;

        println!("配置: {}", config);
        Ok(())
    }

    // 测试
    match process() {
        Ok(_) => println!("处理成功"),
        Err(e) => println!("错误: {:?}", e),  // 会显示完整的错误链
    }

    println!("anyhow 适合应用程序（不是库）");
    println!("可以包装任何错误类型");
    Ok(())
}

/// 示例 10: anyhow 的上下文功能
///
/// anyhow 的 context 方法可以为错误添加上下文信息
/// 这样可以构建一个错误链，从高层到低层逐步说明错误原因
///
/// 错误链的好处：
/// - 保留原始错误信息
/// - 添加业务层面的上下文
/// - 方便调试和定位问题
pub fn anyhow_context() -> AnyhowResult<()> {
    println!("\n=== 示例 10: anyhow 的上下文功能 ===");

    // 读取文件的函数，演示如何添加上下文
    fn read_file(path: &str) -> AnyhowResult<String> {
        // 尝试打开文件
        // File::open 返回 Result<File, io::Error>
        // .context() 添加上下文信息，说明是在打开哪个文件时出错
        let mut file = File::open(path)
            .context(format!("无法打开文件: {}", path))?;

        // 读取文件内容
        let mut contents = String::new();
        // .context() 再次添加上下文，说明是在读取内容时出错
        file.read_to_string(&mut contents)
            .context("读取文件内容失败")?;

        Ok(contents)
    }

    // 尝试读取一个不存在的文件
    match read_file("不存在的文件.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => {
            // 打印错误链
            println!("错误链:");
            println!("  {}", e);  // 最外层的错误（最后添加的上下文）

            // .chain() 返回一个迭代器，包含所有的错误
            // .skip(1) 跳过第一个（因为已经打印过了）
            for cause in e.chain().skip(1) {
                println!("  原因: {}", cause);  // 打印底层的错误原因
            }
            // 输出示例：
            // 错误链:
            //   无法打开文件: 不存在的文件.txt
            //   原因: 系统找不到指定的文件。 (os error 2)
        }
    }

    println!("context 方法添加错误上下文");
    println!("chain 方法遍历错误链");
    Ok(())
}

/// 示例 11: 错误类型的组合
pub fn error_composition() {
    println!("\n=== 示例 11: 错误类型的组合 ===");

    #[derive(Error, Debug)]
    enum AppError {
        #[error("数据库错误: {0}")]
        Database(String),

        #[error("网络错误: {0}")]
        Network(String),

        #[error("业务逻辑错误: {0}")]
        Business(String),
    }

    fn validate_user(age: i32) -> Result<(), AppError> {
        if age < 18 {
            return Err(AppError::Business(String::from("年龄必须大于等于 18")));
        }
        Ok(())
    }

    fn save_user(age: i32) -> Result<(), AppError> {
        validate_user(age)?;
        // 模拟保存到数据库
        println!("  用户已保存（年龄: {}）", age);
        Ok(())
    }

    println!("保存用户（年龄 25）:");
    match save_user(25) {
        Ok(_) => println!("  成功"),
        Err(e) => println!("  失败: {}", e),
    }

    println!("保存用户（年龄 15）:");
    match save_user(15) {
        Ok(_) => println!("  成功"),
        Err(e) => println!("  失败: {}", e),
    }

    println!("组合不同类型的错误到一个枚举");
}

/// 示例 12: Option 和 Result 的转换
///
/// Option 和 Result 可以相互转换
/// - Option -> Result: 使用 ok_or 或 ok_or_else
/// - Result -> Option: 使用 ok 方法
pub fn option_result_conversion() {
    println!("\n=== 示例 12: Option 和 Result 的转换 ===");

    // ========== Option -> Result ==========

    // ok_or: 将 Option 转换为 Result
    // Some(value) -> Ok(value)
    // None -> Err(提供的错误值)
    let some_value: Option<i32> = Some(42);
    let result = some_value.ok_or("值不存在");
    println!("Some(42).ok_or(...): {:?}", result);  // Ok(42)

    let none_value: Option<i32> = None;
    let result = none_value.ok_or("值不存在");
    println!("None.ok_or(...): {:?}", result);  // Err("值不存在")

    // ok_or_else: 和 ok_or 类似，但错误值是延迟计算的
    // 只有在 None 时才会调用闭包
    // 这在错误值的构造比较昂贵时很有用
    let result = none_value.ok_or_else(|| format!("在 {} 时刻值不存在", "现在"));
    println!("None.ok_or_else(...): {:?}", result);

    // ========== Result -> Option ==========

    // ok: 将 Result 转换为 Option
    // Ok(value) -> Some(value)
    // Err(_) -> None（错误信息被丢弃）
    let ok_result: Result<i32, &str> = Ok(42);
    let option = ok_result.ok();
    println!("Ok(42).ok(): {:?}", option);  // Some(42)

    let err_result: Result<i32, &str> = Err("错误");
    let option = err_result.ok();
    println!("Err(...).ok(): {:?}", option);  // None（错误信息丢失）

    println!("ok_or 和 ok 方法用于类型转换");
}

/// 示例 13: 多个错误的处理策略
///
/// 当处理多个可能失败的操作时，有两种常见策略：
/// 1. 全部成功或失败（fail-fast）：遇到第一个错误就停止
/// 2. 收集所有结果和错误：继续处理，最后汇总
pub fn multiple_errors_strategy() {
    println!("\n=== 示例 13: 多个错误的处理策略 ===");

    // 策略 1: 全部成功或失败（fail-fast）
    // 遇到第一个错误就立即返回
    fn process_all_or_nothing(numbers: Vec<&str>) -> Result<Vec<i32>, ParseIntError> {
        // 使用 map 将每个字符串转换为 Result<i32, ParseIntError>
        // collect 会尝试收集所有结果
        // 如果遇到任何 Err，collect 会立即返回那个 Err
        // 只有所有都是 Ok 时，才返回 Ok(Vec<i32>)
        numbers.iter()
            .map(|s| s.parse::<i32>())
            .collect()  // Result<Vec<i32>, ParseIntError>
    }

    // 策略 2: 收集所有结果和错误
    // 即使遇到错误也继续处理，最后返回所有成功的结果和所有错误
    fn process_collect_errors(numbers: Vec<&str>) -> (Vec<i32>, Vec<ParseIntError>) {
        let mut results = Vec::new();  // 存储成功的结果
        let mut errors = Vec::new();   // 存储所有的错误

        // 逐个处理，不会因为错误而停止
        for s in numbers {
            match s.parse::<i32>() {
                Ok(n) => results.push(n),   // 成功就加入结果
                Err(e) => errors.push(e),   // 失败就记录错误
            }
        }

        (results, errors)  // 返回两个 Vec
    }

    // 测试数据：包含有效和无效的字符串
    let data = vec!["1", "2", "abc", "4", "def"];

    println!("全部成功或失败策略:");
    match process_all_or_nothing(data.clone()) {
        Ok(nums) => println!("  成功: {:?}", nums),
        Err(e) => println!("  失败: {}", e),  // 会在 "abc" 处失败
    }

    println!("收集所有结果和错误:");
    let (results, errors) = process_collect_errors(data);
    println!("  成功: {:?}", results);  // [1, 2, 4]
    println!("  错误数: {}", errors.len());  // 2 ("abc" 和 "def")

    println!("根据需求选择合适的错误处理策略");
}

/// 示例 14: 实际应用 - 配置文件解析
///
/// 这个示例展示如何使用 anyhow 解析配置文件
/// 演示了如何为每个可能失败的步骤添加上下文信息
pub fn practical_config_parser() -> AnyhowResult<()> {
    println!("\n=== 示例 14: 实际应用 - 配置文件解析 ===");

    // 定义配置结构体
    #[derive(Debug)]
    struct Config {
        host: String,   // 主机地址
        port: u16,      // 端口号（0-65535）
        timeout: u64,   // 超时时间（秒）
    }

    // 解析配置文件内容
    // 配置格式：每行一个配置项
    // 第 1 行：host
    // 第 2 行：port
    // 第 3 行：timeout
    fn parse_config(content: &str) -> AnyhowResult<Config> {
        // 将内容按行分割
        let lines: Vec<&str> = content.lines().collect();

        // 解析 host
        // .get(0) 返回 Option<&&str>
        // .context() 为 None 的情况添加错误信息
        // ? 将 None 转换为 Err 并返回
        let host = lines.get(0)
            .context("缺少 host 配置")?
            .trim()        // 去除首尾空白
            .to_string();  // 转换为 String

        // 解析 port
        // 需要两次 .context()：
        // 1. 第一次处理 get(1) 可能返回 None
        // 2. 第二次处理 parse() 可能失败
        let port = lines.get(1)
            .context("缺少 port 配置")?
            .trim()
            .parse::<u16>()  // 解析为 u16，可能失败
            .context("port 必须是有效的端口号")?;

        // 解析 timeout
        let timeout = lines.get(2)
            .context("缺少 timeout 配置")?
            .trim()
            .parse::<u64>()
            .context("timeout 必须是有效的数字")?;

        // 构造并返回 Config
        Ok(Config { host, port, timeout })
    }

    // 测试有效的配置
    let valid_config = "localhost\n8080\n30";
    match parse_config(valid_config) {
        Ok(config) => println!("配置解析成功: {:?}", config),
        Err(e) => println!("配置解析失败: {}", e),
    }

    // 测试无效的配置（port 不是数字）
    let invalid_config = "localhost\nabc\n30";
    match parse_config(invalid_config) {
        Ok(config) => println!("配置解析成功: {:?}", config),
        Err(e) => println!("配置解析失败: {}", e),  // 会显示 "port 必须是有效的端口号"
    }

    Ok(())
}

/// 示例 15: 实际应用 - 用户输入验证
pub fn practical_user_validation() {
    println!("\n=== 示例 15: 实际应用 - 用户输入验证 ===");

    #[derive(Error, Debug)]
    enum ValidationError {
        #[error("用户名太短（最少 3 个字符）")]
        UsernameTooShort,

        #[error("用户名太长（最多 20 个字符）")]
        UsernameTooLong,

        #[error("邮箱格式无效")]
        InvalidEmail,

        #[error("年龄必须在 {min} 到 {max} 之间")]
        InvalidAge { min: u8, max: u8 },
    }

    fn validate_username(username: &str) -> Result<(), ValidationError> {
        if username.len() < 3 {
            return Err(ValidationError::UsernameTooShort);
        }
        if username.len() > 20 {
            return Err(ValidationError::UsernameTooLong);
        }
        Ok(())
    }

    fn validate_email(email: &str) -> Result<(), ValidationError> {
        if !email.contains('@') {
            return Err(ValidationError::InvalidEmail);
        }
        Ok(())
    }

    fn validate_age(age: u8) -> Result<(), ValidationError> {
        if age < 18 || age > 120 {
            return Err(ValidationError::InvalidAge { min: 18, max: 120 });
        }
        Ok(())
    }

    fn validate_user(username: &str, email: &str, age: u8) -> Result<(), ValidationError> {
        validate_username(username)?;
        validate_email(email)?;
        validate_age(age)?;
        Ok(())
    }

    println!("验证有效用户:");
    match validate_user("alice", "alice@example.com", 25) {
        Ok(_) => println!("  验证通过"),
        Err(e) => println!("  验证失败: {}", e),
    }

    println!("验证无效用户名:");
    match validate_user("ab", "alice@example.com", 25) {
        Ok(_) => println!("  验证通过"),
        Err(e) => println!("  验证失败: {}", e),
    }

    println!("验证无效邮箱:");
    match validate_user("alice", "invalid-email", 25) {
        Ok(_) => println!("  验证通过"),
        Err(e) => println!("  验证失败: {}", e),
    }

    println!("验证无效年龄:");
    match validate_user("alice", "alice@example.com", 15) {
        Ok(_) => println!("  验证通过"),
        Err(e) => println!("  验证失败: {}", e),
    }
}

/// 示例 16: 实际应用 - 链式错误处理
///
/// 这个示例展示如何在多步骤的处理流程中使用 anyhow
/// 每一步都可能失败，错误会被传播并添加上下文
pub fn practical_error_chain() -> AnyhowResult<()> {
    println!("\n=== 示例 16: 实际应用 - 链式错误处理 ===");

    // 步骤 1: 解析数字
    fn step1() -> AnyhowResult<i32> {
        "42".parse::<i32>()
            .context("步骤 1: 解析数字失败")  // 添加上下文
    }

    // 步骤 2: 验证并处理数字
    fn step2(x: i32) -> AnyhowResult<i32> {
        if x > 100 {
            // anyhow::bail! 是一个宏，用于立即返回错误
            // 等价于 return Err(anyhow!("..."))
            anyhow::bail!("步骤 2: 数字太大");
        }
        Ok(x * 2)
    }

    // 步骤 3: 格式化结果
    fn step3(x: i32) -> AnyhowResult<String> {
        Ok(format!("结果: {}", x))
    }

    // 处理管道：依次执行三个步骤
    // 任何一步失败都会导致整个管道失败
    fn process_pipeline() -> AnyhowResult<String> {
        // 执行步骤 1，如果失败添加 "处理管道失败" 上下文
        let x = step1().context("处理管道失败")?;

        // 执行步骤 2
        let y = step2(x).context("处理管道失败")?;

        // 执行步骤 3
        let result = step3(y).context("处理管道失败")?;

        Ok(result)
    }

    // 运行管道
    match process_pipeline() {
        Ok(result) => println!("{}", result),
        Err(e) => {
            // 打印顶层错误
            println!("错误: {}", e);

            // 打印完整的错误链
            println!("错误链:");
            // .chain() 返回一个迭代器，包含所有层级的错误
            // .enumerate() 添加索引
            for (i, cause) in e.chain().enumerate() {
                println!("  {}: {}", i, cause);
            }
            // 输出示例：
            // 0: 处理管道失败
            // 1: 步骤 1: 解析数字失败
            // 2: invalid digit found in string
        }
    }

    Ok(())
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 错误处理教学代码               ║");
    println!("╚════════════════════════════════════════╝");

    panic_basics();
    unwrap_and_expect();
    result_basics();
    result_methods();
    let _ = question_mark_basics();
    let _ = question_mark_conversion();
    custom_error_manual();
    thiserror_example();
    let _ = anyhow_example();
    let _ = anyhow_context();
    error_composition();
    option_result_conversion();
    multiple_errors_strategy();
    let _ = practical_config_parser();
    practical_user_validation();
    let _ = practical_error_chain();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  错误处理是 Rust 程序的重要组成！   ║");
    println!("╚════════════════════════════════════════╝");
}


