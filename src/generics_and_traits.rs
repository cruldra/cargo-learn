//! # 泛型与 Trait 教学模块
//! 
//! 本模块包含 Rust 泛型和 Trait 的详细教学示例

// 导入标准库中的常用 trait
use std::fmt::{Display, Debug};  // Display 用于格式化输出，Debug 用于调试输出
use std::ops::Add;  // Add trait 用于实现 + 运算符

/// 示例 1: 泛型函数基础
/// 
/// 泛型允许我们编写可以处理多种类型的代码
/// 使用 <T> 声明类型参数，T 是一个占位符，代表任意类型
pub fn generic_functions() {
    println!("\n=== 示例 1: 泛型函数基础 ===");
    
    // 泛型函数：找出两个值中较大的一个
    // <T: PartialOrd> 是 trait bound（特征约束）
    // 表示 T 必须实现 PartialOrd trait（可以比较大小）
    fn largest<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a  // 如果 a 更大，返回 a
        } else {
            b  // 否则返回 b
        }
    }
    
    // 使用泛型函数处理不同类型
    let num1 = 5;
    let num2 = 10;
    println!("较大的数字: {}", largest(num1, num2));  // 自动推断为 i32
    
    let char1 = 'a';
    let char2 = 'z';
    println!("较大的字符: {}", largest(char1, char2));  // 自动推断为 char
    
    println!("泛型函数可以处理多种类型");
}

/// 示例 2: 泛型结构体
/// 
/// 结构体也可以使用泛型，让结构体能够存储不同类型的数据
pub fn generic_structs() {
    println!("\n=== 示例 2: 泛型结构体 ===");
    
    // 定义一个泛型结构体 Point
    // <T> 表示 x 和 y 字段的类型是相同的泛型类型 T
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // 创建不同类型的 Point
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    
    // 多个泛型参数
    // <T, U> 表示 x 和 y 可以是不同的类型
    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }
    
    let mixed_point = Point2 { x: 5, y: 4.0 };  // x 是 i32，y 是 f64
    println!("混合类型点: {:?}", mixed_point);
    
    println!("泛型结构体可以存储不同类型的数据");
}

/// 示例 3: 泛型枚举
/// 
/// 枚举也可以使用泛型，Option 和 Result 就是标准库中的泛型枚举
pub fn generic_enums() {
    println!("\n=== 示例 3: 泛型枚举 ===");
    
    // Option<T> 的定义（标准库中的）
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    
    let some_number: Option<i32> = Some(5);
    let some_string: Option<String> = Some(String::from("hello"));
    let no_value: Option<i32> = None;
    
    println!("Some 数字: {:?}", some_number);
    println!("Some 字符串: {:?}", some_string);
    println!("None: {:?}", no_value);
    
    // Result<T, E> 的定义（标准库中的）
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    
    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err(String::from("错误"));
    
    println!("成功: {:?}", success);
    println!("失败: {:?}", failure);
    
    println!("Option 和 Result 是最常用的泛型枚举");
}

/// 示例 4: 泛型方法
/// 
/// 为泛型结构体实现方法
pub fn generic_methods() {
    println!("\n=== 示例 4: 泛型方法 ===");
    
    // 定义泛型结构体
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // 为泛型结构体实现方法
    // impl<T> 表示这是一个泛型实现
    impl<T> Point<T> {
        // 创建新的 Point
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
        
        // 获取 x 的引用
        fn x(&self) -> &T {
            &self.x
        }
    }
    
    // 只为特定类型实现方法
    // 这里只为 Point<f64> 实现 distance_from_origin 方法
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    
    let p1 = Point::new(5, 10);
    println!("点 p1: {:?}", p1);
    println!("p1.x = {}", p1.x());
    
    let p2 = Point::new(3.0, 4.0);
    println!("点 p2: {:?}", p2);
    println!("p2 到原点的距离: {}", p2.distance_from_origin());
    
    println!("可以为所有泛型类型或特定类型实现方法");
}

/// 示例 5: Trait 基础
/// 
/// Trait 定义了类型必须提供的功能
/// 类似于其他语言中的接口（interface）
pub fn trait_basics() {
    println!("\n=== 示例 5: Trait 基础 ===");
    
    // 定义一个 trait
    // trait 定义了一组方法签名
    trait Summary {
        // 方法签名（没有实现）
        fn summarize(&self) -> String;
        
        // 也可以提供默认实现
        fn author(&self) -> String {
            String::from("未知作者")
        }
    }
    
    // 定义结构体
    struct Article {
        title: String,
        content: String,
        author: String,
    }
    
    struct Tweet {
        username: String,
        content: String,
    }
    
    // 为 Article 实现 Summary trait
    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("《{}》 - {}", self.title, self.author)
        }
        
        // 覆盖默认实现
        fn author(&self) -> String {
            self.author.clone()
        }
    }
    
    // 为 Tweet 实现 Summary trait
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("@{}: {}", self.username, self.content)
        }
        // 使用默认的 author 实现
    }
    
    let article = Article {
        title: String::from("Rust 编程"),
        content: String::from("Rust 是一门系统编程语言..."),
        author: String::from("张三"),
    };
    
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Rust 1.70 发布了！"),
    };
    
    println!("文章摘要: {}", article.summarize());
    println!("文章作者: {}", article.author());
    println!("推文摘要: {}", tweet.summarize());
    println!("推文作者: {}", tweet.author());
    
    println!("Trait 定义共享行为");
}

/// 示例 6: Trait 作为参数
/// 
/// 可以使用 trait 作为函数参数，接受任何实现了该 trait 的类型
pub fn trait_as_parameters() {
    println!("\n=== 示例 6: Trait 作为参数 ===");
    
    trait Summary {
        fn summarize(&self) -> String;
    }
    
    struct Article {
        title: String,
    }
    
    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("文章: {}", self.title)
        }
    }
    
    // 方式 1: impl Trait 语法
    // 接受任何实现了 Summary trait 的类型
    fn notify(item: &impl Summary) {
        println!("通知: {}", item.summarize());
    }
    
    // 方式 2: Trait Bound 语法（更灵活）
    // <T: Summary> 表示 T 必须实现 Summary trait
    fn notify2<T: Summary>(item: &T) {
        println!("通知2: {}", item.summarize());
    }
    
    // 多个 trait bound
    // T 必须同时实现 Summary 和 Display
    fn notify3<T: Summary + Display>(item: &T) {
        println!("通知3: {}", item.summarize());
        println!("显示: {}", item);
    }
    
    let article = Article {
        title: String::from("Rust 学习"),
    };
    
    notify(&article);
    notify2(&article);
    // notify3(&article);  // 编译错误：Article 没有实现 Display
    
    println!("Trait 可以作为函数参数");
}

/// 示例 7: where 子句
/// 
/// 当 trait bound 很多时，使用 where 子句可以让代码更清晰
pub fn where_clause() {
    println!("\n=== 示例 7: where 子句 ===");
    
    // 不使用 where 子句（难以阅读）
    fn _complex_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> String {
        format!("{:?}", u)
    }
    
    // 使用 where 子句（更清晰）
    fn complex_function<T, U>(t: &T, u: &U) -> String
    where
        T: Display + Clone,  // T 必须实现 Display 和 Clone
        U: Clone + Debug,    // U 必须实现 Clone 和 Debug
    {
        format!("{:?}", u)
    }
    
    let num = 42;
    let text = String::from("hello");
    
    let result = complex_function(&num, &text);
    println!("结果: {}", result);
    
    println!("where 子句让复杂的 trait bound 更易读");
}

/// 示例 8: 返回实现了 Trait 的类型
///
/// 函数可以返回实现了某个 trait 的类型
pub fn returning_traits() {
    println!("\n=== 示例 8: 返回实现了 Trait 的类型 ===");

    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Article {
        title: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("文章: {}", self.title)
        }
    }

    // 返回实现了 Summary trait 的类型
    // impl Trait 语法只能返回单一类型
    fn create_summary() -> impl Summary {
        Article {
            title: String::from("Rust 新闻"),
        }
    }

    let summary = create_summary();
    println!("{}", summary.summarize());

    println!("impl Trait 可以用于返回类型");
}

/// 示例 9: 使用 Trait Bound 有条件地实现方法
///
/// 可以根据泛型类型是否实现了某些 trait 来有条件地实现方法
pub fn conditional_trait_implementation() {
    println!("\n=== 示例 9: 使用 Trait Bound 有条件地实现方法 ===");

    #[derive(Debug)]
    struct Pair<T> {
        first: T,
        second: T,
    }

    // 为所有类型实现 new 方法
    impl<T> Pair<T> {
        fn new(first: T, second: T) -> Self {
            Pair { first, second }
        }
    }

    // 只为实现了 Display 和 PartialOrd 的类型实现 cmp_display 方法
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.first >= self.second {
                println!("最大值是 first = {}", self.first);
            } else {
                println!("最大值是 second = {}", self.second);
            }
        }
    }

    let pair1 = Pair::new(5, 10);
    pair1.cmp_display();  // i32 实现了 Display 和 PartialOrd

    let pair2 = Pair::new("hello", "world");
    pair2.cmp_display();  // &str 也实现了这些 trait

    // let pair3 = Pair::new(vec![1], vec![2]);
    // pair3.cmp_display();  // 编译错误：Vec 没有实现 Display

    println!("可以根据 trait bound 有条件地实现方法");
}

/// 示例 10: 派生 Trait
///
/// 使用 #[derive] 属性可以自动实现一些常用的 trait
pub fn derived_traits() {
    println!("\n=== 示例 10: 派生 Trait ===");

    // 自动派生多个 trait
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Person {
        name: String,
        age: u32,
    }

    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Debug trait - 用于调试输出
    println!("Debug: {:?}", person1);

    // Clone trait - 用于克隆
    let person2 = person1.clone();
    println!("克隆: {:?}", person2);

    // PartialEq trait - 用于相等比较
    println!("相等: {}", person1 == person2);

    // PartialOrd trait - 用于大小比较
    let person3 = Person {
        name: String::from("Bob"),
        age: 25,
    };
    println!("比较: {}", person1 > person3);

    println!("常用 trait 可以自动派生:");
    println!("  Debug - 调试输出");
    println!("  Clone - 克隆");
    println!("  PartialEq/Eq - 相等比较");
    println!("  PartialOrd/Ord - 大小比较");
    println!("  Copy - 栈上复制");
}

/// 示例 11: 运算符重载
///
/// 通过实现相应的 trait 可以重载运算符
pub fn operator_overloading() {
    println!("\n=== 示例 11: 运算符重载 ===");

    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    // 实现 Add trait 来重载 + 运算符
    // Add<Rhs> 中的 Rhs 是右操作数的类型
    impl Add for Point {
        type Output = Point;  // 定义返回类型

        // 实现 add 方法
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // 使用 + 运算符

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p1 + p2 = {:?}", p3);

    println!("常见的运算符 trait:");
    println!("  Add (+), Sub (-), Mul (*), Div (/)");
    println!("  Neg (-), Not (!)");
    println!("  Index ([])");
}

/// 示例 12: 关联类型
///
/// 关联类型是 trait 中的类型占位符
pub fn associated_types() {
    println!("\n=== 示例 12: 关联类型 ===");

    // 定义一个带关联类型的 trait
    trait Container {
        type Item;  // 关联类型

        fn add(&mut self, item: Self::Item);
        fn get(&self, index: usize) -> Option<&Self::Item>;
    }

    // 为 Vec<T> 实现 Container
    impl<T> Container for Vec<T> {
        type Item = T;  // 指定关联类型

        fn add(&mut self, item: T) {
            self.push(item);
        }

        fn get(&self, index: usize) -> Option<&T> {
            // 使用切片的 get 方法，避免递归调用
            <[T]>::get(self, index)
        }
    }

    let mut numbers: Vec<i32> = Vec::new();
    numbers.add(1);
    numbers.add(2);
    numbers.add(3);

    println!("第 0 个元素: {:?}", numbers.get(0));
    println!("第 1 个元素: {:?}", numbers.get(1));

    println!("关联类型让 trait 更灵活");
}

/// 示例 13: 默认泛型参数
///
/// trait 可以有默认的泛型参数
pub fn default_generic_parameters() {
    println!("\n=== 示例 13: 默认泛型参数 ===");

    // Add trait 的实际定义
    // trait Add<Rhs = Self> {  // Rhs 默认是 Self
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    #[derive(Debug)]
    struct Millimeters(u32);

    #[derive(Debug)]
    struct Meters(u32);

    // 实现 Add，使用默认的 Rhs = Self
    impl Add for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Millimeters) -> Millimeters {
            Millimeters(self.0 + other.0)
        }
    }

    // 实现 Add，指定不同的 Rhs
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let mm1 = Millimeters(100);
    let mm2 = Millimeters(200);
    let result1 = mm1 + mm2;
    println!("100mm + 200mm = {:?}", result1);

    let mm3 = Millimeters(500);
    let m1 = Meters(2);
    let result2 = mm3 + m1;
    println!("500mm + 2m = {:?}", result2);

    println!("默认泛型参数提供了灵活性");
}

/// 示例 14: Trait 继承
///
/// 一个 trait 可以继承另一个 trait
pub fn trait_inheritance() {
    println!("\n=== 示例 14: Trait 继承 ===");

    // 基础 trait
    trait Animal {
        fn name(&self) -> &str;
    }

    // 继承 Animal trait
    // 实现 Dog trait 的类型必须也实现 Animal trait
    trait Dog: Animal {
        fn bark(&self) {
            println!("{} 说: 汪汪!", self.name());
        }
    }

    struct GoldenRetriever {
        name: String,
    }

    // 必须先实现 Animal
    impl Animal for GoldenRetriever {
        fn name(&self) -> &str {
            &self.name
        }
    }

    // 然后才能实现 Dog
    impl Dog for GoldenRetriever {}

    let dog = GoldenRetriever {
        name: String::from("旺财"),
    };

    println!("狗的名字: {}", dog.name());
    dog.bark();

    println!("Trait 可以继承其他 trait");
}

/// 示例 15: 完全限定语法
///
/// 当多个 trait 有同名方法时，需要使用完全限定语法
pub fn fully_qualified_syntax() {
    println!("\n=== 示例 15: 完全限定语法 ===");

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("机长说：准备起飞！");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("巫师说：飞起来！");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("人类说：我不会飞！");
        }
    }

    let person = Human;

    // 调用 Human 自己的方法
    person.fly();

    // 使用完全限定语法调用 trait 方法
    Pilot::fly(&person);
    Wizard::fly(&person);

    // 也可以这样写
    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);

    println!("完全限定语法: <Type as Trait>::method()");
}

/// 示例 16: 实际应用 - 图形系统
///
/// 使用泛型和 trait 构建一个简单的图形系统
pub fn practical_graphics_system() {
    println!("\n=== 示例 16: 实际应用 - 图形系统 ===");

    // 定义 Shape trait
    trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;

        fn describe(&self) -> String {
            format!(
                "面积: {:.2}, 周长: {:.2}",
                self.area(),
                self.perimeter()
            )
        }
    }

    // 圆形
    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn perimeter(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    // 矩形
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }
    }

    // 打印任何实现了 Shape trait 的类型
    fn print_shape_info(shape: &impl Shape) {
        println!("  {}", shape.describe());
    }

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    println!("圆形:");
    print_shape_info(&circle);

    println!("矩形:");
    print_shape_info(&rectangle);

    println!("Trait 让不同类型共享行为");
}

/// 示例 17: 实际应用 - 泛型容器
///
/// 创建一个泛型容器，可以存储任何类型的数据
pub fn practical_generic_container() {
    println!("\n=== 示例 17: 实际应用 - 泛型容器 ===");

    // 泛型栈
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Stack { items: Vec::new() }
        }

        fn push(&mut self, item: T) {
            self.items.push(item);
        }

        fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }

        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        fn size(&self) -> usize {
            self.items.len()
        }
    }

    // 只为实现了 Display 的类型添加 print 方法
    impl<T: Display> Stack<T> {
        fn print(&self) {
            print!("栈内容: [");
            for (i, item) in self.items.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", item);
            }
            println!("]");
        }
    }

    // 使用整数栈
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);

    println!("整数栈:");
    int_stack.print();
    println!("弹出: {:?}", int_stack.pop());
    int_stack.print();

    // 使用字符串栈
    let mut string_stack = Stack::new();
    string_stack.push(String::from("hello"));
    string_stack.push(String::from("world"));

    println!("\n字符串栈:");
    string_stack.print();
    println!("大小: {}", string_stack.size());

    println!("泛型让代码可以复用");
}

/// 示例 18: 实际应用 - 比较器
///
/// 使用 trait 实现自定义比较逻辑
pub fn practical_comparator() {
    println!("\n=== 示例 18: 实际应用 - 比较器 ===");

    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    // 定义比较器 trait
    trait Comparator<T> {
        fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
    }

    // 按年龄比较
    struct AgeComparator;

    impl Comparator<Person> for AgeComparator {
        fn compare(&self, a: &Person, b: &Person) -> std::cmp::Ordering {
            a.age.cmp(&b.age)
        }
    }

    // 按名字比较
    struct NameComparator;

    impl Comparator<Person> for NameComparator {
        fn compare(&self, a: &Person, b: &Person) -> std::cmp::Ordering {
            a.name.cmp(&b.name)
        }
    }

    // 泛型排序函数
    fn sort_by<T, C>(items: &mut [T], comparator: &C)
    where
        T: Clone,
        C: Comparator<T>,
    {
        items.sort_by(|a, b| comparator.compare(a, b));
    }

    let mut people = vec![
        Person {
            name: String::from("张三"),
            age: 30,
        },
        Person {
            name: String::from("李四"),
            age: 25,
        },
        Person {
            name: String::from("王五"),
            age: 35,
        },
    ];

    println!("原始顺序: {:?}", people);

    sort_by(&mut people, &AgeComparator);
    println!("按年龄排序: {:?}", people);

    sort_by(&mut people, &NameComparator);
    println!("按名字排序: {:?}", people);

    println!("Trait 可以定义自定义行为");
}

/// 运行所有示例
pub fn run_all_examples() {
    println!("\n");
    println!("╔════════════════════════════════════════╗");
    println!("║  Rust 泛型与 Trait 教学代码          ║");
    println!("╚════════════════════════════════════════╝");

    generic_functions();
    generic_structs();
    generic_enums();
    generic_methods();
    trait_basics();
    trait_as_parameters();
    where_clause();
    returning_traits();
    conditional_trait_implementation();
    derived_traits();
    operator_overloading();
    associated_types();
    default_generic_parameters();
    trait_inheritance();
    fully_qualified_syntax();
    practical_graphics_system();
    practical_generic_container();
    practical_comparator();

    println!("\n");
    println!("╔════════════════════════════════════════╗");
    println!("║  泛型和 Trait 是 Rust 的核心特性！  ║");
    println!("╚════════════════════════════════════════╝");
    println!();
}

