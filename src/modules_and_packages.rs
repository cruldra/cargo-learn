/// Rust 模块系统与包管理教学代码
///
/// 模块系统用于组织代码，控制作用域和私有性
/// 包管理系统用于管理依赖和发布代码

/// 示例 1: 模块基础 - 内联模块
///
/// 使用 mod 关键字定义模块
pub fn inline_modules() {
    println!("\n=== 示例 1: 模块基础 - 内联模块 ===");

    // 定义一个模块
    mod greetings {
        // 模块内的函数默认是私有的
        fn private_hello() {
            println!("这是私有函数");
        }

        // 使用 pub 使函数公开
        pub fn hello() {
            println!("Hello from greetings module!");
            // 模块内部可以访问私有函数
            private_hello();
        }

        pub fn goodbye() {
            println!("Goodbye from greetings module!");
        }
    }

    // 使用模块中的公开函数
    greetings::hello();
    greetings::goodbye();
    // greetings::private_hello(); // 错误！私有函数不能访问

    println!("\n模块基础:");
    println!("  - mod 关键字定义模块");
    println!("  - 默认所有内容都是私有的");
    println!("  - pub 关键字使内容公开");
    println!("  - :: 用于访问模块内容");
}

/// 示例 2: 嵌套模块
///
/// 模块可以嵌套定义
pub fn nested_modules() {
    println!("\n=== 示例 2: 嵌套模块 ===");

    mod restaurant {
        // 前台模块
        pub mod front_of_house {
            pub mod hosting {
                pub fn add_to_waitlist() {
                    println!("添加到等待列表");
                }

                pub fn seat_at_table() {
                    println!("安排座位");
                }
            }

            pub mod serving {
                pub fn take_order() {
                    println!("接受订单");
                }

                pub fn serve_order() {
                    println!("上菜");
                }

                pub fn take_payment() {
                    println!("收款");
                }
            }
        }

        // 后厨模块
        mod back_of_house {
            pub fn cook_order() {
                println!("烹饪订单");
            }

            fn prepare_ingredients() {
                println!("准备食材");
            }
        }

        // 餐厅的公开接口
        pub fn eat_at_restaurant() {
            // 相对路径
            front_of_house::hosting::add_to_waitlist();
            front_of_house::hosting::seat_at_table();
            front_of_house::serving::take_order();

            // 访问同级模块
            back_of_house::cook_order();

            front_of_house::serving::serve_order();
            front_of_house::serving::take_payment();
        }
    }

    restaurant::eat_at_restaurant();

    println!("\n嵌套模块:");
    println!("  - 模块可以嵌套任意层级");
    println!("  - 使用 :: 访问嵌套模块");
    println!("  - 绝对路径从 crate 开始");
    println!("  - 相对路径从当前模块开始");
}

/// 示例 3: use 关键字
///
/// 使用 use 简化路径
pub fn use_keyword() {
    println!("\n=== 示例 3: use 关键字 ===");

    mod math {
        pub mod geometry {
            pub fn circle_area(radius: f64) -> f64 {
                std::f64::consts::PI * radius * radius
            }

            pub fn rectangle_area(width: f64, height: f64) -> f64 {
                width * height
            }
        }

        pub mod algebra {
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }

            pub fn multiply(a: i32, b: i32) -> i32 {
                a * b
            }
        }
    }

    // 使用 use 引入模块
    use math::geometry;
    use math::algebra;

    println!("圆面积: {}", geometry::circle_area(5.0));
    println!("矩形面积: {}", geometry::rectangle_area(4.0, 6.0));
    println!("加法: {}", algebra::add(3, 5));
    println!("乘法: {}", algebra::multiply(3, 5));

    // 使用 use 引入具体函数
    use math::geometry::circle_area;
    println!("圆面积: {}", circle_area(3.0));

    println!("\nuse 关键字:");
    println!("  - 简化长路径");
    println!("  - 可以引入模块或具体项");
    println!("  - 作用域仅限当前块");
}

/// 示例 4: use 的高级用法
///
/// as、pub use、嵌套路径
pub fn use_advanced() {
    println!("\n=== 示例 4: use 的高级用法 ===");

    mod utils {
        pub mod string_utils {
            pub fn to_uppercase(s: &str) -> String {
                s.to_uppercase()
            }

            pub fn to_lowercase(s: &str) -> String {
                s.to_lowercase()
            }
        }

        pub mod number_utils {
            pub fn is_even(n: i32) -> bool {
                n % 2 == 0
            }

            pub fn is_odd(n: i32) -> bool {
                n % 2 != 0
            }
        }
    }

    // 使用 as 重命名
    use utils::string_utils as str_utils;
    println!("大写: {}", str_utils::to_uppercase("hello"));

    // 嵌套路径
    use utils::{string_utils, number_utils};
    println!("小写: {}", string_utils::to_lowercase("WORLD"));
    println!("是偶数: {}", number_utils::is_even(4));

    // 使用 * 引入所有公开项
    use utils::number_utils::*;
    println!("是奇数: {}", is_odd(5));

    println!("\nuse 高级用法:");
    println!("  - as: 重命名避免冲突");
    println!("  - {{...}}: 嵌套路径");
    println!("  - *: 引入所有公开项（glob）");
}

/// 示例 5: pub use 重导出
///
/// 使用 pub use 重新导出项
pub fn pub_use_reexport() {
    println!("\n=== 示例 5: pub use 重导出 ===");

    mod library {
        mod internal {
            pub fn helper_function() {
                println!("内部辅助函数");
            }
        }

        // 重导出内部函数，使其在 library 层级可用
        pub use internal::helper_function;

        pub fn public_api() {
            println!("公开 API");
            helper_function();
        }
    }

    // 可以直接从 library 访问重导出的函数
    library::helper_function();
    library::public_api();

    println!("\npub use 重导出:");
    println!("  - 简化外部使用者的路径");
    println!("  - 隐藏内部模块结构");
    println!("  - 提供更好的 API 设计");
}

/// 示例 6: 可见性控制 - pub 的不同级别
///
/// pub、pub(crate)、pub(super)、pub(in path)
pub fn visibility_control() {
    println!("\n=== 示例 6: 可见性控制 - pub 的不同级别 ===");

    mod outer {
        pub fn outer_function() {
            println!("外层函数");
        }

        pub(crate) fn crate_function() {
            println!("crate 级别可见");
        }

        mod inner {
            pub fn inner_function() {
                println!("内层函数");
            }

            pub(super) fn super_function() {
                println!("父模块可见");
            }

            pub(crate) fn limited_function() {
                println!("crate 内可见");
            }

            fn private_function() {
                println!("私有函数");
            }
        }

        pub fn test_visibility() {
            inner::inner_function();
            inner::super_function(); // 可以访问，因为是父模块
            inner::limited_function(); // 可以访问
            // inner::private_function(); // 错误！私有函数
        }
    }

    outer::outer_function();
    outer::crate_function();
    outer::test_visibility();

    println!("\n可见性级别:");
    println!("  - pub: 完全公开");
    println!("  - pub(crate): 当前 crate 内可见");
    println!("  - pub(super): 父模块可见");
    println!("  - pub(in path): 指定路径内可见");
    println!("  - 默认: 私有");
}

/// 示例 7: 结构体和枚举的可见性
///
/// 结构体字段和枚举变体的可见性控制
pub fn struct_enum_visibility() {
    println!("\n=== 示例 7: 结构体和枚举的可见性 ===");

    mod shapes {
        // 公开的结构体
        pub struct Circle {
            pub radius: f64,      // 公开字段
            center_x: f64,        // 私有字段
            center_y: f64,        // 私有字段
        }

        impl Circle {
            // 构造函数
            pub fn new(radius: f64, x: f64, y: f64) -> Circle {
                Circle {
                    radius,
                    center_x: x,
                    center_y: y,
                }
            }

            pub fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }

            pub fn center(&self) -> (f64, f64) {
                (self.center_x, self.center_y)
            }
        }

        // 公开的枚举（所有变体自动公开）
        pub enum Color {
            Red,
            Green,
            Blue,
            RGB(u8, u8, u8),
        }
    }

    let circle = shapes::Circle::new(5.0, 0.0, 0.0);
    println!("半径: {}", circle.radius);
    // println!("中心 X: {}", circle.center_x); // 错误！私有字段
    println!("中心: {:?}", circle.center());
    println!("面积: {}", circle.area());

    let color = shapes::Color::RGB(255, 0, 0);
    match color {
        shapes::Color::Red => println!("红色"),
        shapes::Color::RGB(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        _ => println!("其他颜色"),
    }

    println!("\n结构体和枚举的可见性:");
    println!("  - 结构体字段默认私有");
    println!("  - 需要单独标记字段为 pub");
    println!("  - 枚举变体自动继承枚举的可见性");
}

/// 示例 8: 文件模块系统
///
/// 说明如何将模块拆分到不同文件
pub fn file_module_system() {
    println!("\n=== 示例 8: 文件模块系统 ===");

    println!("文件模块系统的组织方式:");
    println!("\n方式 1: 单文件模块");
    println!("  src/");
    println!("  ├── main.rs");
    println!("  └── my_module.rs  // mod my_module;");
    
    println!("\n方式 2: 目录模块（旧风格）");
    println!("  src/");
    println!("  ├── main.rs");
    println!("  └── my_module/");
    println!("      ├── mod.rs     // 模块根");
    println!("      ├── sub1.rs");
    println!("      └── sub2.rs");

    println!("\n方式 3: 目录模块（新风格，推荐）");
    println!("  src/");
    println!("  ├── main.rs");
    println!("  ├── my_module.rs   // 模块声明");
    println!("  └── my_module/");
    println!("      ├── sub1.rs");
    println!("      └── sub2.rs");

    println!("\n在 main.rs 中:");
    println!("  mod my_module;  // 声明模块");
    println!("  use my_module::some_function;");

    println!("\n在 my_module.rs 中:");
    println!("  pub mod sub1;  // 声明子模块");
    println!("  pub mod sub2;");
    println!("  pub fn some_function() {{}}");
}

/// 示例 9: super 和 self 关键字
///
/// 使用 super 和 self 进行相对路径引用
pub fn super_and_self() {
    println!("\n=== 示例 9: super 和 self 关键字 ===");

    mod parent {
        pub fn parent_function() {
            println!("父模块函数");
        }

        pub mod child {
            pub fn child_function() {
                println!("子模块函数");

                // 使用 super 访问父模块
                super::parent_function();

                // 使用 self 访问当前模块
                self::helper();
            }

            fn helper() {
                println!("辅助函数");
            }
        }

        pub mod sibling {
            pub fn sibling_function() {
                println!("兄弟模块函数");

                // 使用 super 访问父模块
                super::parent_function();

                // 通过父模块访问兄弟模块
                super::child::child_function();
            }
        }
    }

    parent::child::child_function();
    println!();
    parent::sibling::sibling_function();

    println!("\nsuper 和 self:");
    println!("  - super: 访问父模块");
    println!("  - self: 访问当前模块");
    println!("  - 用于相对路径引用");
}

/// 示例 10: Cargo.toml 基础
///
/// 说明 Cargo.toml 的基本配置
pub fn cargo_toml_basics() {
    println!("\n=== 示例 10: Cargo.toml 基础 ===");

    println!("Cargo.toml 是 Rust 项目的配置文件\n");

    println!("[package] - 包的元数据");
    println!("  name = \"my-project\"      # 包名");
    println!("  version = \"0.1.0\"         # 版本号");
    println!("  edition = \"2021\"          # Rust 版本");
    println!("  authors = [\"Your Name\"]   # 作者");
    println!("  license = \"MIT\"           # 许可证");
    println!("  description = \"...\"       # 描述");
    println!("  repository = \"...\"        # 仓库地址");
    println!("  keywords = [\"...\"]        # 关键词");
    println!("  categories = [\"...\"]      # 分类");

    println!("\n[dependencies] - 依赖");
    println!("  serde = \"1.0\"             # 简单版本");
    println!("  tokio = {{ version = \"1.0\", features = [\"full\"] }}");
    println!("  rand = \"0.8\"");

    println!("\n[dev-dependencies] - 开发依赖");
    println!("  criterion = \"0.5\"         # 基准测试");
    println!("  proptest = \"1.0\"          # 属性测试");

    println!("\n[build-dependencies] - 构建依赖");
    println!("  cc = \"1.0\"                # C 编译器");

    println!("\n版本号规则:");
    println!("  - 语义化版本: major.minor.patch");
    println!("  - ^1.2.3: >=1.2.3 且 <2.0.0");
    println!("  - ~1.2.3: >=1.2.3 且 <1.3.0");
    println!("  - 1.2.*: >=1.2.0 且 <1.3.0");
    println!("  - =1.2.3: 精确版本");
}

/// 示例 11: 依赖来源
///
/// 说明不同的依赖来源
pub fn dependency_sources() {
    println!("\n=== 示例 11: 依赖来源 ===");

    println!("1. crates.io（默认）");
    println!("  [dependencies]");
    println!("  serde = \"1.0\"");

    println!("\n2. Git 仓库");
    println!("  [dependencies]");
    println!("  my-lib = {{ git = \"https://github.com/user/repo\" }}");
    println!("  my-lib = {{ git = \"...\", branch = \"main\" }}");
    println!("  my-lib = {{ git = \"...\", tag = \"v1.0\" }}");
    println!("  my-lib = {{ git = \"...\", rev = \"abc123\" }}");

    println!("\n3. 本地路径");
    println!("  [dependencies]");
    println!("  my-lib = {{ path = \"../my-lib\" }}");

    println!("\n4. 组合使用");
    println!("  [dependencies]");
    println!("  serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("  tokio = {{ version = \"1.0\", default-features = false }}");

    println!("\n特性（Features）:");
    println!("  - 条件编译功能");
    println!("  - 减小编译体积");
    println!("  - 可选依赖");
}

/// 示例 12: 工作空间（Workspace）
///
/// 说明如何使用工作空间管理多个包
pub fn workspace_basics() {
    println!("\n=== 示例 12: 工作空间（Workspace） ===");

    println!("工作空间用于管理多个相关的包\n");

    println!("项目结构:");
    println!("  my-workspace/");
    println!("  ├── Cargo.toml          # 工作空间配置");
    println!("  ├── Cargo.lock          # 统一的锁文件");
    println!("  ├── target/             # 共享的构建目录");
    println!("  ├── common/             # 共享库");
    println!("  │   ├── Cargo.toml");
    println!("  │   └── src/lib.rs");
    println!("  ├── server/             # 服务端");
    println!("  │   ├── Cargo.toml");
    println!("  │   └── src/main.rs");
    println!("  └── client/             # 客户端");
    println!("      ├── Cargo.toml");
    println!("      └── src/main.rs");

    println!("\n根目录 Cargo.toml:");
    println!("  [workspace]");
    println!("  members = [");
    println!("      \"common\",");
    println!("      \"server\",");
    println!("      \"client\",");
    println!("  ]");
    println!("  resolver = \"2\"");

    println!("\n在成员包中引用其他成员:");
    println!("  # server/Cargo.toml");
    println!("  [dependencies]");
    println!("  common = {{ path = \"../common\" }}");

    println!("\n工作空间的优势:");
    println!("  - 共享依赖版本");
    println!("  - 统一的 Cargo.lock");
    println!("  - 共享构建缓存");
    println!("  - 方便管理相关项目");
}

/// 示例 13: 发布到 crates.io
///
/// 说明如何发布包到 crates.io
pub fn publishing_to_crates_io() {
    println!("\n=== 示例 13: 发布到 crates.io ===");

    println!("发布前的准备:\n");

    println!("1. 完善 Cargo.toml");
    println!("  [package]");
    println!("  name = \"my-awesome-crate\"");
    println!("  version = \"0.1.0\"");
    println!("  edition = \"2021\"");
    println!("  authors = [\"Your Name <you@example.com>\"]");
    println!("  license = \"MIT OR Apache-2.0\"");
    println!("  description = \"A short description\"");
    println!("  repository = \"https://github.com/user/repo\"");
    println!("  documentation = \"https://docs.rs/my-awesome-crate\"");
    println!("  readme = \"README.md\"");
    println!("  keywords = [\"keyword1\", \"keyword2\"]");
    println!("  categories = [\"category1\"]");

    println!("\n2. 添加文档注释");
    println!("  /// 这是一个公开函数");
    println!("  /// ");
    println!("  /// # Examples");
    println!("  /// ");
    println!("  /// ```");
    println!("  /// use my_crate::my_function;");
    println!("  /// assert_eq!(my_function(2), 4);");
    println!("  /// ```");
    println!("  pub fn my_function(x: i32) -> i32 {{ x * 2 }}");

    println!("\n3. 发布步骤");
    println!("  $ cargo login <your-api-token>");
    println!("  $ cargo publish --dry-run  # 测试发布");
    println!("  $ cargo publish            # 正式发布");

    println!("\n4. 版本管理");
    println!("  - 遵循语义化版本");
    println!("  - 0.x.y: 开发阶段");
    println!("  - 1.0.0: 稳定版本");
    println!("  - 发布后不能删除或修改");

    println!("\n注意事项:");
    println!("  - 包名必须唯一");
    println!("  - 必须有许可证");
    println!("  - 建议添加 README.md");
    println!("  - 建议添加示例和文档");
}

/// 示例 14: Cargo 命令
///
/// 常用的 Cargo 命令
pub fn cargo_commands() {
    println!("\n=== 示例 14: Cargo 命令 ===");

    println!("项目管理:");
    println!("  cargo new <name>           # 创建新项目");
    println!("  cargo new --lib <name>     # 创建库项目");
    println!("  cargo init                 # 在当前目录初始化");

    println!("\n构建和运行:");
    println!("  cargo build                # 调试构建");
    println!("  cargo build --release      # 发布构建");
    println!("  cargo run                  # 构建并运行");
    println!("  cargo run --release        # 发布模式运行");
    println!("  cargo check                # 快速检查（不生成可执行文件）");

    println!("\n测试:");
    println!("  cargo test                 # 运行所有测试");
    println!("  cargo test <name>          # 运行特定测试");
    println!("  cargo test --release       # 发布模式测试");
    println!("  cargo bench                # 运行基准测试");

    println!("\n文档:");
    println!("  cargo doc                  # 生成文档");
    println!("  cargo doc --open           # 生成并打开文档");

    println!("\n依赖管理:");
    println!("  cargo add <crate>          # 添加依赖");
    println!("  cargo remove <crate>       # 移除依赖");
    println!("  cargo update               # 更新依赖");
    println!("  cargo tree                 # 显示依赖树");

    println!("\n发布:");
    println!("  cargo publish              # 发布到 crates.io");
    println!("  cargo yank --vers <ver>    # 撤回版本");

    println!("\n其他:");
    println!("  cargo clean                # 清理构建产物");
    println!("  cargo fmt                  # 格式化代码");
    println!("  cargo clippy               # 代码检查");
}

/// 示例 15: 条件编译
///
/// 使用 cfg 进行条件编译
pub fn conditional_compilation() {
    println!("\n=== 示例 15: 条件编译 ===");

    // 根据操作系统编译不同代码
    #[cfg(target_os = "windows")]
    fn platform_specific() {
        println!("这是 Windows 平台");
    }

    #[cfg(target_os = "linux")]
    fn platform_specific() {
        println!("这是 Linux 平台");
    }

    #[cfg(target_os = "macos")]
    fn platform_specific() {
        println!("这是 macOS 平台");
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    fn platform_specific() {
        println!("这是其他平台");
    }

    platform_specific();

    // 根据特性编译
    #[cfg(feature = "advanced")]
    fn advanced_feature() {
        println!("高级特性已启用");
    }

    #[cfg(not(feature = "advanced"))]
    fn advanced_feature() {
        println!("高级特性未启用");
    }

    advanced_feature();

    // 调试模式 vs 发布模式
    #[cfg(debug_assertions)]
    println!("调试模式");

    #[cfg(not(debug_assertions))]
    println!("发布模式");

    println!("\n条件编译:");
    println!("  - #[cfg(...)]: 条件编译属性");
    println!("  - target_os: 目标操作系统");
    println!("  - target_arch: 目标架构");
    println!("  - feature: 特性标志");
    println!("  - debug_assertions: 调试断言");
}

/// 示例 16: 自定义特性（Features）
///
/// 定义和使用自定义特性
pub fn custom_features() {
    println!("\n=== 示例 16: 自定义特性（Features） ===");

    println!("在 Cargo.toml 中定义特性:\n");

    println!("[features]");
    println!("default = [\"std\"]          # 默认特性");
    println!("std = []                    # 标准库支持");
    println!("serde = [\"dep:serde\"]      # 可选的 serde 支持");
    println!("advanced = [\"std\", \"serde\"] # 组合特性");

    println!("\n[dependencies]");
    println!("serde = {{ version = \"1.0\", optional = true }}");

    println!("\n在代码中使用:");
    println!("  #[cfg(feature = \"serde\")]");
    println!("  use serde::{{Serialize, Deserialize}};");
    println!("  ");
    println!("  #[cfg_attr(feature = \"serde\", derive(Serialize, Deserialize))]");
    println!("  pub struct MyStruct {{}}");

    println!("\n启用特性:");
    println!("  cargo build --features serde");
    println!("  cargo build --features \"serde,advanced\"");
    println!("  cargo build --all-features");
    println!("  cargo build --no-default-features");

    println!("\n在依赖中启用特性:");
    println!("  [dependencies]");
    println!("  my-crate = {{ version = \"1.0\", features = [\"serde\"] }}");
}

/// 示例 17: 实际应用 - 库的组织
///
/// 展示一个实际库的模块组织
pub fn practical_library_organization() {
    println!("\n=== 示例 17: 实际应用 - 库的组织 ===");

    // 模拟一个数学库的组织
    mod math_lib {
        // 基础模块
        pub mod basic {
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }

            pub fn subtract(a: i32, b: i32) -> i32 {
                a - b
            }
        }

        // 高级模块
        pub mod advanced {
            pub fn power(base: f64, exp: f64) -> f64 {
                base.powf(exp)
            }

            pub fn sqrt(x: f64) -> f64 {
                x.sqrt()
            }
        }

        // 几何模块
        pub mod geometry {
            pub struct Point {
                pub x: f64,
                pub y: f64,
            }

            impl Point {
                pub fn new(x: f64, y: f64) -> Self {
                    Point { x, y }
                }

                pub fn distance(&self, other: &Point) -> f64 {
                    let dx = self.x - other.x;
                    let dy = self.y - other.y;
                    (dx * dx + dy * dy).sqrt()
                }
            }
        }

        // 预导入模块（prelude）
        pub mod prelude {
            pub use super::basic::*;
            pub use super::geometry::Point;
        }
    }

    // 使用预导入
    use math_lib::prelude::*;

    println!("加法: {}", add(3, 5));
    println!("减法: {}", subtract(10, 4));

    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("距离: {}", p1.distance(&p2));

    // 使用高级功能
    use math_lib::advanced;
    println!("幂运算: {}", advanced::power(2.0, 3.0));

    println!("\n库的组织最佳实践:");
    println!("  - 按功能划分模块");
    println!("  - 提供 prelude 模块");
    println!("  - 使用 pub use 重导出");
    println!("  - 隐藏内部实现细节");
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Rust 模块系统与包管理教学代码        ║");
    println!("╚════════════════════════════════════════╝");

    inline_modules();
    nested_modules();
    use_keyword();
    use_advanced();
    pub_use_reexport();
    visibility_control();
    struct_enum_visibility();
    file_module_system();
    super_and_self();
    cargo_toml_basics();
    dependency_sources();
    workspace_basics();
    publishing_to_crates_io();
    cargo_commands();
    conditional_compilation();
    custom_features();
    practical_library_organization();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  模块系统是组织大型项目的关键！      ║");
    println!("╚════════════════════════════════════════╝\n");
}

