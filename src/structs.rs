// Rust 结构体教学代码
// 主题：结构体定义、实例化、方法、关联函数、元组结构体、单元结构体

/// 示例 1: 基本结构体定义与实例化
pub fn basic_struct() {
    println!("\n=== 示例 1: 基本结构体定义与实例化 ===");
    
    // 定义结构体
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    // 创建结构体实例
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    println!("登录次数: {}", user1.sign_in_count);
    println!("是否激活: {}", user1.active);
}

/// 示例 2: 可变结构体
pub fn mutable_struct() {
    println!("\n=== 示例 2: 可变结构体 ===");
    
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("修改前 - 邮箱: {}", user1.email);
    
    user1.email = String::from("newemail@example.com");
    user1.sign_in_count += 1;
    
    println!("修改后 - 邮箱: {}", user1.email);
    println!("登录次数: {}", user1.sign_in_count);
    println!("注意：整个实例必须是可变的，不能只让某些字段可变");
}

/// 示例 3: 字段初始化简写
pub fn field_init_shorthand() {
    println!("\n=== 示例 3: 字段初始化简写 ===");
    
    struct User {
        username: String,
        email: String,
        active: bool,
    }
    
    fn build_user(email: String, username: String) -> User {
        User {
            email,      // 简写，等同于 email: email
            username,   // 简写，等同于 username: username
            active: true,
        }
    }
    
    let user = build_user(
        String::from("user@example.com"),
        String::from("someuser123")
    );
    
    println!("用户名: {}", user.username);
    println!("邮箱: {}", user.email);
}

/// 示例 4: 结构体更新语法
pub fn struct_update_syntax() {
    println!("\n=== 示例 4: 结构体更新语法 ===");
    
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };
    
    // 使用 ..user1 从 user1 复制其余字段
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1  // user1 的 username 被移动到 user2
    };
    
    println!("user2 邮箱: {}", user2.email);
    println!("user2 用户名: {}", user2.username);
    // println!("{}", user1.username);  // 错误！username 已被移动
    println!("user1.email 仍然有效: {}", user1.email);
    println!("user1.active 仍然有效: {}", user1.active);
}

/// 示例 5: 元组结构体
pub fn tuple_structs() {
    println!("\n=== 示例 5: 元组结构体 ===");
    
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("黑色 RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("原点坐标: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // let color: Color = origin;  // 错误！即使结构相同，类型也不同
    
    println!("元组结构体有名称，但字段没有名称");
}

/// 示例 6: 单元结构体
pub fn unit_like_structs() {
    println!("\n=== 示例 6: 单元结构体 ===");
    
    struct AlwaysEqual;
    
    let subject = AlwaysEqual;
    
    println!("单元结构体没有任何字段");
    println!("常用于实现 trait 但不需要存储数据的场景");
    
    // 避免未使用警告
    let _subject = subject;
}

/// 示例 7: 方法定义
pub fn methods() {
    println!("\n=== 示例 7: 方法定义 ===");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // 方法：第一个参数是 &self
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }
        
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    
    println!("矩形面积: {}", rect1.area());
    println!("矩形周长: {}", rect1.perimeter());
    println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));
}

/// 示例 8: 关联函数
pub fn associated_functions() {
    println!("\n=== 示例 8: 关联函数 ===");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // 关联函数：不接收 self 参数
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(20);
    
    println!("矩形: {}x{}", rect.width, rect.height);
    println!("正方形: {}x{}", sq.width, sq.height);
    println!("关联函数使用 :: 语法调用，类似其他语言的静态方法");
}

/// 示例 9: 多个 impl 块
pub fn multiple_impl_blocks() {
    println!("\n=== 示例 9: 多个 impl 块 ===");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    impl Rectangle {
        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }
    }
    
    let rect = Rectangle { width: 30, height: 50 };
    
    println!("面积: {}", rect.area());
    println!("周长: {}", rect.perimeter());
    println!("可以为同一个结构体定义多个 impl 块");
}

/// 示例 10: 方法的所有权
pub fn method_ownership() {
    println!("\n=== 示例 10: 方法的所有权 ===");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // 不可变借用
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // 可变借用
        fn scale(&mut self, factor: u32) {
            self.width *= factor;
            self.height *= factor;
        }
        
        // 获取所有权
        fn into_square(self) -> Rectangle {
            let size = self.width.max(self.height);
            Rectangle { width: size, height: size }
        }
    }
    
    let mut rect = Rectangle { width: 10, height: 20 };
    
    println!("原始面积: {}", rect.area());
    
    rect.scale(2);
    println!("缩放后面积: {}", rect.area());
    
    let square = rect.into_square();
    println!("转换为正方形: {}x{}", square.width, square.height);
    // println!("{}", rect.width);  // 错误！rect 已被移动
}

/// 示例 11: 派生 trait
pub fn derived_traits() {
    println!("\n=== 示例 11: 派生 trait ===");
    
    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 5, y: 10 };
    let p2 = p1.clone();
    
    println!("p1: {:?}", p1);  // Debug trait
    println!("p2: {:?}", p2);
    println!("p1 == p2: {}", p1 == p2);  // PartialEq trait
    println!("使用 #[derive] 自动实现常用 trait");
}

/// 示例 12: 嵌套结构体
pub fn nested_structs() {
    println!("\n=== 示例 12: 嵌套结构体 ===");

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 10, y: 0 },
    };

    println!("矩形: {:?}", rect);
    println!("左上角: ({}, {})", rect.top_left.x, rect.top_left.y);
    println!("右下角: ({}, {})", rect.bottom_right.x, rect.bottom_right.y);
}

/// 示例 13: 结构体与所有权
pub fn struct_ownership() {
    println!("\n=== 示例 13: 结构体与所有权 ===");

    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        age: u32,
    }

    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 25,
    };

    // 部分移动：String 字段被移动，基本类型字段被复制
    let username = user1.username;
    let age = user1.age;

    println!("用户名: {}", username);
    println!("年龄: {}", age);
    // println!("{:?}", user1);  // 错误！username 已被移动
    println!("email 仍然有效: {}", user1.email);
}

/// 示例 14: 实际应用 - 图书管理
pub fn practical_book_management() {
    println!("\n=== 示例 14: 实际应用 - 图书管理 ===");

    #[derive(Debug, Clone)]
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    impl Book {
        fn new(title: String, author: String, pages: u32) -> Self {
            Book {
                title,
                author,
                pages,
                available: true,
            }
        }

        fn borrow_book(&mut self) -> bool {
            if self.available {
                self.available = false;
                true
            } else {
                false
            }
        }

        fn return_book(&mut self) {
            self.available = true;
        }

        fn info(&self) -> String {
            format!(
                "《{}》 - {} ({} 页) [{}]",
                self.title,
                self.author,
                self.pages,
                if self.available { "可借" } else { "已借出" }
            )
        }
    }

    let mut book = Book::new(
        String::from("Rust 编程语言"),
        String::from("Steve Klabnik"),
        500
    );

    println!("{}", book.info());

    if book.borrow_book() {
        println!("借阅成功");
    }
    println!("{}", book.info());

    book.return_book();
    println!("归还后: {}", book.info());
}

/// 示例 15: 实际应用 - 银行账户
pub fn practical_bank_account() {
    println!("\n=== 示例 15: 实际应用 - 银行账户 ===");

    struct BankAccount {
        account_number: String,
        holder_name: String,
        balance: f64,
    }

    impl BankAccount {
        fn new(account_number: String, holder_name: String) -> Self {
            BankAccount {
                account_number,
                holder_name,
                balance: 0.0,
            }
        }

        fn deposit(&mut self, amount: f64) {
            if amount > 0.0 {
                self.balance += amount;
                println!("存入 {:.2} 元，余额: {:.2} 元", amount, self.balance);
            }
        }

        fn withdraw(&mut self, amount: f64) -> bool {
            if amount > 0.0 && amount <= self.balance {
                self.balance -= amount;
                println!("取出 {:.2} 元，余额: {:.2} 元", amount, self.balance);
                true
            } else {
                println!("余额不足或金额无效");
                false
            }
        }

        fn get_balance(&self) -> f64 {
            self.balance
        }
    }

    let mut account = BankAccount::new(
        String::from("6222021234567890"),
        String::from("张三")
    );

    println!("账户持有人: {}", account.holder_name);
    println!("账号: {}", account.account_number);

    account.deposit(1000.0);
    account.withdraw(300.0);
    account.withdraw(800.0);

    println!("最终余额: {:.2} 元", account.get_balance());
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 结构体教学代码                 ║");
    println!("╚════════════════════════════════════════╝");

    basic_struct();
    mutable_struct();
    field_init_shorthand();
    struct_update_syntax();
    tuple_structs();
    unit_like_structs();
    methods();
    associated_functions();
    multiple_impl_blocks();
    method_ownership();
    derived_traits();
    nested_structs();
    struct_ownership();
    practical_book_management();
    practical_bank_account();

    println!("\n╔════════════════════════════════════════╗");
    println!("║  结构体是组织相关数据的强大工具！   ║");
    println!("╚════════════════════════════════════════╝");
}


