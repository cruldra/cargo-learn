// Rust 注释与文档注释教学代码
// 主题：普通注释、文档注释、文档测试

/// 示例 1: 普通注释
/// Rust 支持两种普通注释：行注释和块注释
pub fn normal_comments() {
    println!("\n=== 示例 1: 普通注释 ===");
    
    // 这是单行注释
    // 使用双斜杠 // 开头
    let x = 5; // 也可以在代码后面添加注释
    
    /* 这是块注释 */
    /* 块注释可以
       跨越多行
       使用 /* */ 包围 */
    let y = 10;
    
    /* 块注释还可以 /* 嵌套 */ 使用 */
    
    println!("x = {}, y = {}", x, y);
    println!("普通注释不会出现在生成的文档中");
}

/// 示例 2: 文档注释（外部文档）
/// 使用三斜杠 /// 为函数、结构体等添加文档
pub fn outer_doc_comments() {
    println!("\n=== 示例 2: 文档注释（外部文档） ===");
    
    /// 计算两个数的和
    /// 
    /// # 参数
    /// 
    /// * `a` - 第一个加数
    /// * `b` - 第二个加数
    /// 
    /// # 返回值
    /// 
    /// 返回两个数的和
    /// 
    /// # 示例
    /// 
    /// ```
    /// let result = add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    let result = add(10, 20);
    println!("10 + 20 = {}", result);
    println!("使用 /// 添加的文档注释会出现在生成的文档中");
}

/// 示例 3: 内部文档注释
/// 使用 //! 为模块或 crate 添加文档
pub fn inner_doc_comments() {
    println!("\n=== 示例 3: 内部文档注释 ===");

    // 内部文档注释示例（在模块内部）
    mod example {
        //! 这是一个内部文档注释
        //! 通常用于模块或 crate 的顶部
        //! 描述整个模块的功能

        pub fn module_fn() {
            println!("模块内的函数");
        }
    }

    example::module_fn();
    println!("内部文档注释使用 //! 开头");
    println!("通常放在文件或模块的开头");
}

/// 示例 4: 文档注释的 Markdown 支持
/// 文档注释支持 Markdown 格式
pub fn markdown_in_docs() {
    println!("\n=== 示例 4: 文档注释的 Markdown 支持 ===");
    
    /// # 这是一级标题
    /// 
    /// ## 这是二级标题
    /// 
    /// 可以使用 **粗体** 和 *斜体*
    /// 
    /// 还可以使用列表：
    /// - 项目 1
    /// - 项目 2
    /// - 项目 3
    /// 
    /// 代码块：
    /// ```
    /// let x = 5;
    /// println!("{}", x);
    /// ```
    /// 
    /// 链接：[Rust 官网](https://www.rust-lang.org/)
    fn documented_function() {
        println!("这个函数有丰富的 Markdown 文档");
    }
    
    documented_function();
    println!("文档注释支持完整的 Markdown 语法");
}

/// 示例 5: 常用文档注释章节
/// 
/// # 参数 (Arguments/Parameters)
/// 
/// 描述函数的参数
/// 
/// # 返回值 (Returns)
/// 
/// 描述函数的返回值
/// 
/// # 示例 (Examples)
/// 
/// 提供使用示例
/// 
/// # Panics
/// 
/// 描述函数可能 panic 的情况
/// 
/// # Errors
/// 
/// 描述函数可能返回的错误
/// 
/// # Safety
/// 
/// 对于 unsafe 函数，描述安全使用的条件
pub fn common_doc_sections() {
    println!("\n=== 示例 5: 常用文档注释章节 ===");
    
    /// 从切片中获取指定索引的元素
    /// 
    /// # 参数
    /// 
    /// * `slice` - 要搜索的切片
    /// * `index` - 要获取的索引
    /// 
    /// # 返回值
    /// 
    /// 返回索引处的元素引用，如果索引越界则返回 None
    /// 
    /// # 示例
    /// 
    /// ```
    /// let numbers = vec![1, 2, 3, 4, 5];
    /// let result = get_element(&numbers, 2);
    /// assert_eq!(result, Some(&3));
    /// ```
    /// 
    /// # Panics
    /// 
    /// 此函数不会 panic
    fn get_element<T>(slice: &[T], index: usize) -> Option<&T> {
        slice.get(index)
    }
    
    let numbers = vec![10, 20, 30];
    if let Some(value) = get_element(&numbers, 1) {
        println!("索引 1 的值: {}", value);
    }
    
    println!("使用标准章节可以让文档更加规范");
}

/// 示例 6: 为结构体添加文档
pub fn struct_documentation() {
    println!("\n=== 示例 6: 为结构体添加文档 ===");
    
    /// 表示一个二维平面上的点
    /// 
    /// # 字段
    /// 
    /// * `x` - 点的 x 坐标
    /// * `y` - 点的 y 坐标
    /// 
    /// # 示例
    /// 
    /// ```
    /// let point = Point { x: 10, y: 20 };
    /// println!("点的坐标: ({}, {})", point.x, point.y);
    /// ```
    struct Point {
        /// x 坐标
        x: i32,
        /// y 坐标
        y: i32,
    }
    
    impl Point {
        /// 创建一个新的点
        /// 
        /// # 参数
        /// 
        /// * `x` - x 坐标
        /// * `y` - y 坐标
        /// 
        /// # 返回值
        /// 
        /// 返回一个新的 Point 实例
        fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
        
        /// 计算点到原点的距离
        /// 
        /// # 返回值
        /// 
        /// 返回距离的平方（避免浮点运算）
        fn distance_squared(&self) -> i32 {
            self.x * self.x + self.y * self.y
        }
    }
    
    let point = Point::new(3, 4);
    println!("点的坐标: ({}, {})", point.x, point.y);
    println!("距离平方: {}", point.distance_squared());
}

/// 示例 7: 为枚举添加文档
pub fn enum_documentation() {
    println!("\n=== 示例 7: 为枚举添加文档 ===");
    
    /// 表示 HTTP 请求方法
    /// 
    /// # 变体
    /// 
    /// * `Get` - GET 请求
    /// * `Post` - POST 请求
    /// * `Put` - PUT 请求
    /// * `Delete` - DELETE 请求
    #[derive(Debug)]
    enum HttpMethod {
        /// GET 请求，用于获取资源
        Get,
        /// POST 请求，用于创建资源
        Post,
        /// PUT 请求，用于更新资源
        Put,
        /// DELETE 请求，用于删除资源
        Delete,
    }
    
    let method = HttpMethod::Get;
    println!("HTTP 方法: {:?}", method);
    println!("枚举的每个变体都可以有自己的文档");
}

/// 示例 8: 文档测试
/// 
/// 文档注释中的代码块会被作为测试运行
/// 
/// # 示例
/// 
/// ```
/// // 这段代码会在 cargo test 时运行
/// let x = 2 + 2;
/// assert_eq!(x, 4);
/// ```
pub fn doc_tests() {
    println!("\n=== 示例 8: 文档测试 ===");
    
    /// 将两个数相加
    /// 
    /// # 示例
    /// 
    /// ```
    /// # fn add(a: i32, b: i32) -> i32 { a + b }
    /// let result = add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    /// 
    /// 也可以展示会失败的情况：
    /// 
    /// ```should_panic
    /// # fn add(a: i32, b: i32) -> i32 { a + b }
    /// let result = add(2, 3);
    /// assert_eq!(result, 6); // 这会 panic
    /// ```
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    println!("结果: {}", add(5, 7));
    println!("文档中的代码示例会在 cargo test 时自动测试");
}

/// 示例 9: 隐藏文档测试中的代码
/// 
/// 使用 # 可以隐藏某些代码行
/// 
/// # 示例
/// 
/// ```
/// # fn setup() {}
/// # fn teardown() {}
/// # setup();
/// let x = 42;
/// println!("{}", x);
/// # teardown();
/// ```
/// 
/// 上面的示例中，setup() 和 teardown() 不会显示在文档中
pub fn hidden_doc_test_lines() {
    println!("\n=== 示例 9: 隐藏文档测试中的代码 ===");
    println!("使用 # 可以隐藏测试辅助代码");
}

/// 示例 10: 忽略文档测试
/// 
/// 有些代码示例不需要测试
/// 
/// # 示例
/// 
/// ```ignore
/// // 这段代码不会被测试
/// let x = some_external_function();
/// ```
/// 
/// 或者标记为编译失败的示例：
/// 
/// ```compile_fail
/// // 这段代码预期编译失败
/// let x: i32 = "not a number";
/// ```
pub fn ignore_doc_tests() {
    println!("\n=== 示例 10: 忽略文档测试 ===");
    println!("使用 ignore 或 compile_fail 标记特殊的代码示例");
}

/// 示例 11: 模块级文档
/// 
/// 通常在文件开头使用 //! 添加模块文档
pub fn module_level_docs() {
    println!("\n=== 示例 11: 模块级文档 ===");
    
    mod example_module {
        //! 这是一个示例模块
        //! 
        //! 这个模块展示了如何使用内部文档注释
        //! 
        //! # 示例
        //! 
        //! ```
        //! // 使用模块中的函数
        //! ```
        
        /// 模块中的函数
        pub fn module_function() {
            println!("这是模块中的函数");
        }
    }
    
    example_module::module_function();
    println!("模块级文档使用 //! 添加");
}

/// 示例 12: 生成文档
/// 
/// 使用 cargo doc 命令生成 HTML 文档
pub fn generating_docs() {
    println!("\n=== 示例 12: 生成文档 ===");
    println!("使用以下命令生成文档：");
    println!("  cargo doc           - 生成文档");
    println!("  cargo doc --open    - 生成文档并在浏览器中打开");
    println!("  cargo test --doc    - 运行文档测试");
    println!("\n文档会生成在 target/doc 目录下");
}

/// 运行所有示例
/// 
/// 这个函数会依次运行所有注释相关的示例
/// 
/// # 示例
/// 
/// ```
/// # fn run_all_examples() {}
/// run_all_examples();
/// ```
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 注释与文档注释教学代码         ║");
    println!("╚════════════════════════════════════════╝");
    
    normal_comments();
    outer_doc_comments();
    inner_doc_comments();
    markdown_in_docs();
    common_doc_sections();
    struct_documentation();
    enum_documentation();
    doc_tests();
    hidden_doc_test_lines();
    ignore_doc_tests();
    module_level_docs();
    generating_docs();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  所有示例运行完毕！                  ║");
    println!("╚════════════════════════════════════════╝");
}

