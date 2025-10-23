# Rust 学习教程

这是一个完整的 Rust 基础教学项目，包含多个教学模块，每个模块都有详细的中文注释和示例代码。

## 📚 教学模块

### 1. 变量与可变性 (`variables_and_mutability.rs`)

学习 Rust 中的变量声明和可变性概念。

**主要内容：**
- 不可变变量（默认行为）
- 可变变量（使用 `mut` 关键字）
- 变量遮蔽（Shadowing）
- 遮蔽 vs 可变性的区别
- 常量（`const`）
- 未使用的变量处理
- 解构赋值
- 可变引用
- 类型推断与显式类型标注
- 延迟初始化
- 作用域与生命周期
- 实际应用场景

### 2. 数据类型 (`data_types.rs`)

学习 Rust 的标量类型和复合类型。

**主要内容：**

**标量类型：**
- 整数类型（i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize）
- 整数字面量（十进制、十六进制、八进制、二进制）
- 浮点数类型（f32, f64）
- 数值运算
- 布尔类型（bool）
- 字符类型（char）

**复合类型：**
- 元组类型
- 数组类型
- 切片类型
- 字符串类型（&str 和 String）

**高级特性：**
- 类型转换
- 类型别名

### 3. 函数定义与调用 (`functions.rs`)

学习 Rust 中的函数定义、参数、返回值等概念。

**主要内容：**
- 基本函数定义与调用
- 带参数的函数
- 带返回值的函数
- 语句与表达式
- 提前返回（`return` 关键字）
- 无返回值函数（单元类型 `()`）
- 函数作为参数（高阶函数）
- 返回函数
- 递归函数（包括尾递归优化）
- 方法与关联函数
- 泛型函数
- 发散函数（`!` 类型）

### 4. 注释与文档注释 (`comments.rs`)

学习 Rust 中的注释和文档注释系统。

**主要内容：**
- 普通注释（行注释和块注释）
- 文档注释（外部文档 `///`）
- 内部文档注释（`//!`）
- 文档注释的 Markdown 支持
- 常用文档注释章节（参数、返回值、示例、Panics、Errors、Safety）
- 为结构体添加文档
- 为枚举添加文档
- 文档测试
- 隐藏文档测试中的代码
- 忽略文档测试
- 模块级文档
- 生成文档

### 5. 控制流 (`control_flow.rs`)

学习 Rust 中的控制流语句和表达式。

**主要内容：**

**条件表达式：**
- if 表达式（if 是表达式，可以返回值）
- if let 表达式（简化的模式匹配）

**循环：**
- loop 无限循环
- loop 返回值
- 循环标签（嵌套循环控制）
- while 条件循环
- while let 循环
- for 循环遍历集合
- for 循环使用范围（`..` 和 `..=`）
- break 和 continue

**模式匹配：**
- match 表达式
- match 守卫（添加额外条件）

**实际应用：**
- 猜数字游戏
- 斐波那契数列
- 九九乘法表

### 6. 所有权规则 (`ownership.rs`)

学习 Rust 最核心、最独特的特性 - 所有权系统。

**主要内容：**

**所有权基础：**
- 所有权三大规则
- 变量与数据的交互 - 移动（Move）
- 变量与数据的交互 - 克隆（Clone）
- 栈上数据的复制（Copy trait）

**所有权与函数：**
- 所有权与函数参数
- 返回值与所有权
- 返回多个值
- 所有权转移的时机

**高级概念：**
- 部分移动（结构体字段）
- 所有权与作用域
- 所有权与 Vec
- 所有权与 Box 智能指针

**实际应用：**
- 字符串拼接（+ 运算符 vs format! 宏）
- 交换值（std::mem::swap）
- 构建器模式

### 7. 引用与借用 (`references_and_borrowing.rs`)

学习 Rust 的引用和借用机制，这是所有权系统的重要补充。

**主要内容：**

**引用基础：**
- 引用基础（使用值但不获取所有权）
- 引用与所有权的对比
- 不可变引用（可以同时存在多个）
- 可变引用（修改借用的值）

**借用规则：**
- 可变引用的限制（同一作用域只能有一个）
- 可变引用与不可变引用不能共存
- 借用规则总结
- 悬垂引用（编译器防止）

**引用的使用：**
- 引用作为函数参数
- 可变引用修改数据
- 引用的作用域（NLL - Non-Lexical Lifetimes）
- 多个可变引用（不同作用域）
- 引用与切片

**实际应用：**
- 查找和替换
- 数据验证

### 8. 结构体 (`structs.rs`)

学习如何使用结构体组织相关数据，这是 Rust 中最重要的数据组织方式。

**主要内容：**

**结构体基础：**
- 基本结构体定义与实例化
- 可变结构体
- 字段初始化简写
- 结构体更新语法

**特殊结构体：**
- 元组结构体（有名称但字段无名称）
- 单元结构体（无字段）

**方法与关联函数：**
- 方法定义（&self, &mut self, self）
- 关联函数（类似静态方法）
- 多个 impl 块
- 方法的所有权

**高级特性：**
- 派生 trait（Debug, Clone, PartialEq）
- 嵌套结构体
- 结构体与所有权

**实际应用：**
- 图书管理系统
- 银行账户管理

### 9. 枚举 (`enums.rs`)

学习 Rust 的枚举类型，这是表达复杂数据和状态的强大工具。

**主要内容：**

**枚举基础：**
- 基本枚举定义
- 带数据的枚举（每个变体可携带不同类型的数据）
- 枚举方法

**Option 和 Result：**
- Option 枚举（表示可能存在或不存在的值）
- Option 的常用方法（unwrap, unwrap_or, map 等）
- Result 枚举（表示成功或失败）
- Result 的常用方法

**模式匹配：**
- if let 简化匹配
- while let 循环
- 模式匹配的强大功能

**高级特性：**
- 枚举与结构体结合
- 递归枚举（使用 Box）
- 枚举的内存布局

**实际应用：**
- 状态机（红绿灯）
- 表达式求值器

### 10. 集合类型 (`collections.rs`)

学习 Rust 标准库中的常用集合类型，这些是构建实际程序的基础工具。

**主要内容：**

**Vector（动态数组）：**
- Vector 基础（创建、访问、修改）
- Vector 的常用操作（push, pop, insert, remove）
- 遍历 Vector（不可变、可变、获取所有权）
- 使用枚举在 Vector 中存储不同类型

**String 和 &str：**
- String 基础（创建、UTF-8 编码）
- String 的操作（追加、拼接、替换）
- String 和 &str 的区别
- 字符串索引和遍历（字符、字节）

**HashMap 和 BTreeMap：**
- HashMap 基础（键值对存储）
- HashMap 的操作（遍历、更新、entry API）
- HashMap 的所有权
- BTreeMap（有序的键值对）

**HashSet 和 BTreeSet：**
- HashSet 基础（唯一值集合）
- HashSet 的集合操作（并集、交集、差集）
- BTreeSet（有序的唯一值集合）

**实际应用：**
- 学生成绩管理
- 去重和排序
- 文本分析（单词频率统计）

### 11. 错误处理 (`error_handling.rs`)

学习 Rust 的错误处理机制，这是编写健壮程序的关键。

**主要内容：**

**panic! 和不可恢复错误：**
- panic! 宏基础
- unwrap 和 expect 方法
- 何时使用 panic

**Result 类型：**
- Result<T, E> 基础
- Result 的常用方法（is_ok, is_err, unwrap_or, map, and_then）
- match 处理 Result

**? 操作符：**
- ? 操作符基础（错误传播）
- ? 操作符的错误转换
- 简化错误处理代码

**自定义错误类型：**
- 手动实现错误类型（Display, Error trait）
- 使用 thiserror 简化错误定义
- 错误类型的组合

**anyhow 库：**
- 使用 anyhow 简化应用程序错误处理
- 添加错误上下文（context 方法）
- 错误链（chain 方法）

**高级技巧：**
- Option 和 Result 的转换
- 多个错误的处理策略

**实际应用：**
- 配置文件解析
- 用户输入验证
- 链式错误处理

## 🚀 使用方法

### 运行教学示例

在 `src/main.rs` 中，你可以选择运行不同的教学模块：

```rust
fn main() {
    // 取消注释你想运行的模块

    // 变量与可变性
    // variables_and_mutability::run_all_examples();

    // 数据类型
    // data_types::run_all_examples();

    // 函数定义与调用
    // functions::run_all_examples();

    // 注释与文档注释
    // comments::run_all_examples();

    // 控制流
    // control_flow::run_all_examples();

    // 所有权规则
    // ownership::run_all_examples();

    // 引用与借用
    // references_and_borrowing::run_all_examples();

    // 结构体
    // structs::run_all_examples();

    // 枚举
    // enums::run_all_examples();

    // 集合类型
    // collections::run_all_examples();

    // 错误处理
    error_handling::run_all_examples();
}
```

然后运行：

```bash
cargo run
```

### 生成文档

生成 HTML 格式的文档：

```bash
# 生成文档
cargo doc

# 生成文档并在浏览器中打开
cargo doc --open

# 只生成本项目的文档（不包括依赖）
cargo doc --no-deps
```

文档会生成在 `target/doc/cargo_learn/index.html`

### 运行文档测试

运行文档注释中的代码示例：

```bash
cargo test --doc
```

## 📖 学习建议

1. **按顺序学习**：建议按照以下顺序学习模块：
   - 变量与可变性
   - 数据类型
   - 函数定义与调用
   - 注释与文档注释
   - 控制流
   - 所有权规则
   - 引用与借用
   - 结构体
   - 枚举
   - 集合类型
   - 错误处理

2. **动手实践**：每个模块都可以独立运行，建议边看代码边运行

3. **阅读注释**：每个示例都有详细的中文注释，仔细阅读可以更好地理解

4. **修改代码**：尝试修改示例代码，观察编译器的错误提示

5. **查看文档**：使用 `cargo doc --open` 查看生成的文档，了解文档注释的效果

## 🛠️ 项目结构

```
cargo-learn/
├── src/
│   ├── main.rs                      # 主程序入口
│   ├── variables_and_mutability.rs  # 变量与可变性教学模块
│   ├── data_types.rs                # 数据类型教学模块
│   ├── functions.rs                 # 函数教学模块
│   ├── comments.rs                  # 注释教学模块
│   ├── control_flow.rs              # 控制流教学模块
│   ├── ownership.rs                 # 所有权规则教学模块
│   ├── references_and_borrowing.rs  # 引用与借用教学模块
│   ├── structs.rs                   # 结构体教学模块
│   ├── enums.rs                     # 枚举教学模块
│   ├── collections.rs               # 集合类型教学模块
│   └── error_handling.rs            # 错误处理教学模块
├── Cargo.toml                       # 项目配置文件
└── README.md                        # 项目说明文档
```

## 📝 代码特点

- ✅ 所有代码都有详细的中文注释
- ✅ 每个模块包含多个独立的示例
- ✅ 示例由浅入深，循序渐进
- ✅ 包含实际应用场景
- ✅ 代码符合 Rust 最佳实践
- ✅ 完整的文档注释

## 🎯 学习目标

通过学习这个项目，你将掌握：

- Rust 的基本语法
- 变量和可变性的概念
- Rust 的类型系统
- 函数的定义和使用
- 如何编写高质量的文档注释
- Rust 的表达式和语句
- 控制流结构（if、loop、while、for、match）
- **所有权系统** - Rust 最核心的特性
- **引用与借用** - 安全地使用值而不获取所有权
- **结构体** - 组织相关数据的强大工具
- **枚举** - 表达复杂数据和状态的利器
- **集合类型** - Vector、String、HashMap、HashSet 等常用数据结构
- **错误处理** - Result、panic、? 操作符、thiserror、anyhow

## 📚 扩展学习

完成这些基础模块后，建议继续学习：

- 模块系统（Modules & Packages - mod, pub, use, crate）
- 泛型（Generics - 泛型函数、泛型结构体、泛型枚举）
- Trait（特征 - 定义共享行为、trait bounds）
- 生命周期（Lifetimes - 引用的有效性、生命周期标注）
- 迭代器（Iterators - map, filter, collect, fold）
- 闭包（Closures - 匿名函数、捕获环境）
- 智能指针（Smart Pointers - Box, Rc, RefCell, Arc）
- 并发编程（Concurrency - 线程、消息传递、共享状态、Mutex）
- 异步编程（Async/Await - Future, async fn, tokio）
- 宏（Macros - 声明宏、过程宏）

## 📄 许可证

本项目仅用于学习目的。

## 🤝 贡献

欢迎提出建议和改进意见！

