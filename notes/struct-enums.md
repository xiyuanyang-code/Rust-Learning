# Rust Tutorial 3: Struct and Enums in Rust

{% note primary %}

**为什么 Rust 没有设计类**？

Rust 没有类的概念，只有结构体（Struct）。从某种程度上讲，Rust 的结构体和传统面向对象语言（如 C++、Java）中的类非常相似，它们都可以用来封装数据。但是，Rust 并没有像类那样的继承（Inheritance）机制。

Rust 的设计哲学倾向于**组合（Composition）优于继承**。在传统的面向对象编程中，类继承常常导致代码库变得复杂且难以维护，因为子类会隐式地继承父类的所有行为和数据。这会形成一个紧密的耦合关系，使得修改父类可能会意外地影响到多个子类。

Rust 鼓励你使用组合的方式来实现类似的功能。你可以通过将一个结构体嵌入到另一个结构体中，或者使用 Trait（特型）来定义共享的行为，从而避免了继承带来的复杂性。

同时，Rust 本身的语言特性也能够带来类似类继承的效果，例如 trait 可以实现特定的方法（功能类似于抽象基类）、同时复杂的类继承关系也可能会影响到 Rust 本身强大且安全的所有权系统。

{% endnote %}

面向对象编程可以让代码更加的解耦并且模块化，是一种良好的代码规范。因此，Rust 也支持创建自定义的类型，可以通过**结构体**和**枚举**的方法创建，同时，Rust 的面向对象是**零成本抽象**并且**内存安全**的，可以在避免处理复杂类继承关系的前提下写出模块化且安全的代码。 

## Struct

### 定义和实例化

非常类似于 Python 的键值对的管理，定义结构体和创建结构体的实例化都非常简单。

```rust
struct User {
    active: bool,
    user_name: String,
    id: i32,
    email: String,
}
fn main() {
    println!("Hello world!");
    create_struct();

    let user_test: User = create_user(String::from("hello@sjtu.edu.cn"), String::from("wow"));
    println!("{}", user_test.active);
}

fn create_struct() {
    // using the key-value pair
    let user_1 = User {
        active: true,
        user_name: String::from("Xiyuan Yang"),
        id: 123456,
        email: String::from("test@gmail.com"),
    };

    let user_2 = User {
        active: false,
        user_name: String::from("test"),
        id: 1233445,
        email: String::from("test"),
    };

    println!("{}{}{}", user_1.active, user_1.id, user_1.email);
    println!("{}", user_2.user_name);
}

// or you can simplify this...

fn create_user(email: String, user_name: String) -> User {
    User {
        active: true,
        user_name,
        id: 12345,
        email,
    }
}
```

同样的，我们可以从一个实例化的对象出发构建第二个实例化的对象，这样的解包语法在 Rust 中是被允许的。

```rust
fn create_user_from_existing(user_1: User) -> User{
    let user_more = User{
        active: false,
        ..user_1
    };
    user_more
}
```

不过注意！这里传入参数传入的是直接 User 类的实例而不是借用，因此会直接交出所有权，传入的实际参数就不可以再使用。（除非显示的使用 clone 方法。）

如何实现这一点，首先需要给自己定义的结构体加上对应的 trait：

```rust
#[derive(Clone)]
struct User {
    active: bool,
    user_name: String,
    id: i32,
    email: String,
}
```

因为这个结构体的每一个类型都有clone方法（尤其指的是 String 类），因此这个结构体成功实现了 Clone Trait，注意**String 类没有 Copy Trait**，因此这个结构体无法实现 Copy Trait。

### 无命名字段结构体创建

以上创建结构体的方式非常像创建了一个 Python 的字典，同样，Rust 提供了类似于**元组**的结构体创建方式，在这样的创建方式下创建不需要提供显式的变量名称（不需要为每个字段命名）。

```rust
#[derive(Clone, Copy)]
struct Color(i32, i32, i32);
#[derive(Clone, Copy)]
struct Point(i32, i32, i32);

let color_test = Color(0, 0, 0);
let point_test = Point(23, 23, 23);

let color_values = [color_test.0, color_test.1, color_test.2];
println!("Color:");
for value in color_values {
    println!("{}", value);
}

let point_values = [point_test.0, point_test.1, point_test.2];
println!("Point:");
for value in point_values {
    println!("{}", value);
}
```

### 类单元结构体

类似于空元组，它的主要用途是实现某个 trait，同时又不需要存储任何数据。（例如发送某些提示信息。）

### 结构体数据的所有权

在上文的解包语法中，直接使用借用来创建一个新的结构体是不被允许的，因为这相当于一个结构体没有其数据成员的所有权。

> 借用的生命周期在它最后一次被使用的地方结束。

为什么？还是经典的**生命周期和悬垂引用的问题**。如果你想让结构体存储被其他对象拥有的数据的引用，你需要使用**生命周期参数（Lifetimes）**。这是 Rust 强制执行内存安全的机制，确保结构体中的引用不会超出它所引用的数据的生命周期。

> 因此这样的使用是被允许的，只是为了保证生命周期不超过被引用的对象，需要手动加入生命周期的参数。

想象一下这个场景：

1.  你有一个函数，在其中创建了一个 `String` 变量。
2.  你创建了一个结构体实例，它包含一个指向这个 `String` 的引用。
3.  函数执行结束，`String` 变量被销毁。
4.  但是，你的结构体实例可能还在某个地方被使用。

这会导致一个悬垂引用（dangling reference）错误，即结构体中的引用指向了一个已经被释放的内存地址。Rust 的借用检查器（borrow checker）会阻止这种情况发生，方法是强制你使用生命周期参数来明确这些引用的有效范围。


#### 生命周期参数

要让一个结构体存储引用，你需要在结构体的定义中使用生命周期参数。生命周期参数通常以 `'a`、`'b` 等形式命名，并放在结构体名称后面的尖括号里。

```rust
// 定义一个名为 'a 的生命周期参数
struct Student<'a> {
    name: &'a str,
    id: u32,
}

fn main() {
    let student_name = String::from("Alice");
    
    // 创建一个 Student 实例
    let student1 = Student {
        name: &student_name, // 引用 student_name
        id: 12345,
    };
    
    // student1 的生命周期不能比 student_name 的生命周期长
    println!("Student name: {}", student1.name);
}
```

在这个例子中：

  * `Student<'a>` 表示 `Student` 结构体有一个生命周期参数 `'a`。
  * `name: &'a str` 表示 `name` 字段是一个字符串切片，并且它的生命周期必须至少与结构体实例的生命周期 `'a` 一样长。
  * `student1` 在 `main` 函数的作用域内被创建，它引用的 `student_name` 也在同一个作用域内。Rust 编译器会自动推断出 `student1` 的生命周期和 `student_name` 的生命周期是相同的，并验证这个借用是安全的。

Rust 的编译器会强制执行以下规则：**包含引用的结构体实例的生命周期不能超过它所引用的数据的生命周期。**

让我们看一个会报错的例子：

```rust
struct Student<'a> {
    name: &'a str,
    id: u32,
}

fn create_student<'a>(name: &'a str) -> Student<'a> {
    let student_id = 12345;
    let student_instance = Student {
        name,
        id: student_id,
    };
    student_instance
}

fn main() {
    let student_instance;
    { // inner scope
        let student_name = String::from("Alice");
        student_instance = create_student(&student_name);
        // 这里 student_name 被销毁，但 student_instance 还在
    } // inner scope ends, student_name is dropped

    // 错误！student_instance.name 变成了悬垂引用
    // 尽管我们声明了生命周期，但借用检查器会发现这里不安全
    println!("Student name: {}", student_instance.name);
}
```

在上面的例子中，`student_name` 在内层作用域结束时就被销毁了。但是，我们试图将 `student_instance` 传递到外层作用域，它的 `name` 字段仍然引用着已被销毁的 `student_name`。借用检查器会立即阻止这段代码编译，并告诉你 **`student_name` 的生命周期太短了**。

例如：

```rust
// 使用字符串切片，因为 str 是 unsized 的，因此必须使用切片，相当于使用指针来存储。
let user = NewUser{
    name: &"hello world"[0..=2],
    email: "wow it is great"
};

println!("{}", user.name);
println!("{}", user.email);
```

> 字符串是**不可变类型但是非固定大小**，但是创建结构体的时候必须要在编译时明确结构体的大小，因此必须使用引用的方式。

### 派生 Trait

当我们想要直接 println! 我们的结构体时，往往会出现报错。原因也很简单，我们没有实现对应的 trait。

```text
error[E0277]: `User` doesn't implement `std::fmt::Display`
  --> src/bin/struct.rs:52:20
   |
52 |     println!("{}", user_test);
   |               --   ^^^^^^^^^ `User` cannot be formatted with the default formatter
   |               |
   |               required by this formatting parameter
   |
   = help: the trait `std::fmt::Display` is not implemented for `User`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```rust
#[derive(Clone, Debug)]
struct User {
    active: bool,
    user_name: String,
    id: i32,
    email: String,
}
```

最终，通过加入 Debug 的 Trait 也可以实现使用 println! 来输出相关变量的信息。

```text
User { active: true, user_name: "wow", id: 12345, email: "hello@sjtu.edu.cn" }
```

{% note primary %}

`Debug` Trait 是 Rust 标准库中一个非常有用的特性，它的主要作用是让开发者能够以一种对人类友好的方式，**格式化和打印**结构体或枚举等复杂数据类型的内容，这在调试时非常方便。

当你使用 `{}` 或 `{:?}` 格式化字符串时，你实际上是在告诉 Rust 使用 `Display` 或 `Debug` Trait 来打印数据。

  * **`{}`**：用于**用户可见**的输出。这需要类型实现 `std::fmt::Display` Trait。比如 `String` 和 `i32` 都实现了这个 Trait。
  * **`{:?}`**：用于**开发者可见**的调试输出。这需要类型实现 `std::fmt::Debug` Trait。

有两种主要方法来实现 `Debug` Trait：

1.  **使用 `#[derive(Debug)]` 派生宏**

    这是最简单、最常见的实现方式。当你将 `#[derive(Debug)]` 放在结构体或枚举的定义上方时，Rust 编译器会自动为你生成实现 `Debug` Trait 所需的代码。这个自动生成的代码会打印出类型名称以及所有字段的名称和值。

    ```rust
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
    }

    fn main() {
        let user = User {
            name: String::from("Alice"),
            age: 30,
        };

        // 使用 {:?} 进行调试打印
        println!("{:?}", user); 

        // 使用 {:#?} 进行美观打印
        println!("{:#?}", user); 
    }
    ```

    **输出：**

    ```text
    User { name: "Alice", age: 30 }
    User {
        name: "Alice",
        age: 30,
    }
    ```

    注意，`{:#?}` 提供了更美观、带有缩进的打印，非常适合复杂的嵌套结构。

2.  **手动实现 `Debug` Trait**

    在某些情况下，你可能不希望默认的 `Debug` 实现暴露所有字段，或者想要自定义输出格式。这时，你可以手动为类型实现 `Debug`。

    ```rust
    use std::fmt;

    struct MySecret {
        secret_data: String,
        public_id: u32,
    }

    impl fmt::Debug for MySecret {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 只打印公共ID，隐藏敏感数据
            f.debug_struct("MySecret")
             .field("public_id", &self.public_id)
             .finish()
        }
    }

    fn main() {
        let secret = MySecret {
            secret_data: String::from("my password"),
            public_id: 123,
        };
        println!("{:?}", secret);
    }
    ```

    **输出：**

    ```text
    MySecret { public_id: 123 }
    ```

{% endnote %}


### 方法

使用结构体可以初步实现面向对象编程的“结构化”，即使用结构体组装成更有意义的文本。而使用**方法**能够实现对应“成员函数”的功能，不过方法和函数存在差别，他们在结构体的上下文中被定义。

例如，对于一个长方形的类，我们可以定义一些方法。这里面的定义方法和 Python 的语法很像，其他的和函数定义无异。

```rust
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Hello world!");
    let reca = Rectangle {
        width: 100,
        height: 100,
    };
    println!("{reca:#?}");
    println!("Area: {}", reca.area());
}
```

#### -> 运算符去哪了？

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect_val = Rectangle { width: 30, height: 50 };
    let rect_ref = &rect_val;

    // 访问 rect_val（一个值）的成员
    println!("Area: {}", rect_val.area());

    // 访问 rect_ref（一个引用）的成员
    // Rust 自动解引用，所以你还是用 . 运算符
    println!("Area: {}", rect_ref.area()); 
}
```

Rust 的编译器非常智能，它会自动处理解引用（dereferencing）。当它看到你在一个引用上使用了`.`运算符来访问其字段或方法时，它会自动在后台将 rect_ref.area() 转换成 (*rect_ref).area()。这种自动解引用的特性让代码更加简洁，并且避免了程序员手动添加 * 和 ->。


具体来说，当看到 `.` 运算符的时候，会自动为 object 添加：

- &
- &mut
- *

来实现方法签名匹配。

因此，对于可变引用也一样：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    let mut rect_val = Rectangle { width: 30, height: 50 };
    let rect_mut_ref = &mut rect_val;

    // 使用 . 运算符调用可变引用上的方法
    rect_mut_ref.set_width(100); 
    
    // 这行代码在后台被 Rust 编译器自动转换为：
    // (*rect_mut_ref).set_width(100);

    println!("New width: {}", rect_val.width);
}
```

至此，方法的定义已经实现，我们可以实现面向对象编程了！

#### 关联函数

定义不以 self 为第一个参数的方法函数，则被称为关联函数，这些函数并不作用于实例上，而是被抽象的结构体所共有。例如，`String::from` 就是一个关联函数的示例。

```rust
#[derive(Debug, Clone, Copy)]
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
    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

impl Rectangle {
    fn square(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
```

## enums

- 结构体的使用逻辑完全类似于面向对象编程的设计。将数据成员（字段）和数据聚合在一起。
- 枚举可以声明**某一个值是一个集合中的一员**。

两者的视角不同，但是都可以提供创建自定义类型的方法。枚举就是“是多个可能状态中的一个”。它让你能够定义一个类型，这个类型的值只能是预先定义好的、有限的几个变体（variant）之一。

例如，在代码中表示 IP 地址，因为要么是 IPv4 要么是 IPv6，因此可以使用枚举值的特性来表示 IP 地址，而具体是 v4 还是 v6 被称为**枚举的变体**。

```rust
#[derive(Debug)]
enum IpAddrkind {
    V4,
    V6,
}

fn main() {
    println!("Hello world");
    let four = IpAddrkind::V4;
    let six = IpAddrkind::V6;
    println!("{four:#?}");
    println!("{six:#?}");
}
```

```text
V4
V6
```

{% note primary %}

### Why enums?

在许多情况下，使用 **枚举（`enum`）** 来表示自定义类型比使用 **结构体（`struct`）** 更有优势。这通常取决于你要建模的数据的本质。


#### 数据互斥性

这是使用枚举最经典、最主要的场景。如果一个值**只能是几种预先定义好的、有限的变体中的一个**，并且这个变体之间是相互互斥的。


一个 `Shape`（形状）要么是圆形，要么是矩形，要么是三角形，但它不可能同时是它们中的两个。

  * **使用枚举：** 完美且类型安全。
    ```rust
    enum Shape {
        Circle(f64), // 圆形带一个半径
        Rectangle(f64, f64), // 矩形带长和宽
        Triangle(f64, f64, f64), // 三角形带三条边
    }
    ```
  * **使用结构体（不推荐）：** 笨拙且容易出错。
    ```rust
    struct ShapeStruct {
        circle_radius: Option<f64>,
        rect_width: Option<f64>,
        rect_height: Option<f64>,
        tri_side1: Option<f64>,
        tri_side2: Option<f64>,
        tri_side3: Option<f64>,
    }
    ```
    在这种结构体设计中，你需要处理大量的 `Option` 类型，并且无法在编译时保证你只初始化了其中一种形状的数据。一个 `ShapeStruct` 的实例可能既有圆形的半径，又有矩形的边长，这在逻辑上是错误的。


#### 多种情况处理

Rust 的 **`match` 表达式** 和枚举是天生一对。`match` 表达式能够强制你处理枚举的所有可能变体，从而保证了代码的完整性和健壮性。

**例子：网络请求结果**

一个网络请求要么成功并返回数据，要么失败并返回一个错误。

  * **使用枚举（`Result` 类型）：** `match` 表达式会强制你处理 `Ok` 和 `Err` 这两种情况。
    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    fn handle_result(res: Result<String, String>) {
        match res {
            Result::Ok(data) => println!("请求成功：{}", data),
            Result::Err(e) => println!("请求失败：{}", e),
        }
    }
    ```
  * **使用结构体（不推荐）：** 你需要手动检查 `is_success` 字段或 `error` 字段，这很容易遗漏。
    ```rust
    struct ApiResponse {
        is_success: bool,
        data: Option<String>,
        error: Option<String>,
    }

    // 开发者很容易忘记检查 is_success
    fn process_response(res: ApiResponse) {
        if let Some(data) = res.data {
            println!("请求成功：{}", data);
        }
    }
    ```
    在这种情况下，如果 `res` 是一个失败的响应（`is_success` 为 `false`），上面的代码会静默失败，因为 `res.data` 是 `None`，这可能导致难以追踪的 bug。

#### 当类型本身是“标签”或“状态”时

如果一个类型只是用来表示一个状态或一个标签，而不需要携带太多数据，枚举能提供更清晰的语义。

**例子：按键事件**

一个按键事件要么是按下，要么是松开。

  * **使用枚举：** 简洁明了。
    ```rust
    enum ButtonState {
        Pressed,
        Released,
    }
    ```
  * **使用结构体（不推荐）：** 显得过于冗余。
    ```rust
    struct ButtonState {
        is_pressed: bool,
    }
    ```
    `ButtonState { is_pressed: true }` 的写法虽然可行，但 `ButtonState::Pressed` 在语义上更具表现力。

| 特性       | **枚举 (`enum`)** | **结构体 (`struct`)** |
| :--------- | :------------------------------------------------- | :-------------------------------------------------- |
| **主要目的** | 建模 **互斥的** 集合或状态。                     | 建模 **聚合的** 数据。                              |
| **核心思想** | “一个值 **是** 几种可能情况中的一个。”          | “一个值 **拥有** 多个数据字段。”                 |
| **优势** | 类型安全，强制性处理所有情况，代码语义更清晰。 | 灵活的数据组合，是 OOP 中“对象”的基础。 |

总而言之，如果你在思考“这个值要么是 A，要么是 B，要么是 C”，那么使用 **枚举**。如果你在思考“这个值由 A、B 和 C 组成”，那么使用 **结构体**。选择正确的工具，能让你的代码更健壮、更易于维护。

{% endnote %}

### 枚举中变量的存储

数据可以直接放进每一个枚举变体中，而不是将枚举作为结构体的一部分，这样在自定义结构的设计上会更加简洁。

> 事实上，在真实情况下，很多人还是会趋于面向对象编程的惯性，选择后者。但是后者意味着编码相同的任务需要自定义两个类型，这在复杂的软件工程管理中会带来冗余的工作量。

```rust
#[derive(Debug, Clone)]
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0,1"));
```

使用枚举而不是结构体的另一个优势是：**每个变体可以处理不同类型和数量的数据**。

{% note primary %}

枚举相比于结构体更加适用于**变体**的设置，结构体要求统一的数据成员设置，而枚举的设定则更加的灵活。

或者换句话说，**枚举**实现了 Rust 中面向对象编程的**多态性**，而这个特性原来是由类继承来保证的。

> 不过这两个概念之间还是有一些区别的，类继承之间的关系为 is-a 的关系，而枚举之间的关系是 has-a 的关系。

在 Rust 中，这种通过枚举来建模互斥类型，并用 match 表达式进行处理的方式，被称为**代数数据类型**（Algebraic Data Types）。这种方法通常比类继承更安全、更清晰，因为它强迫你显式地处理所有可能的变体，避免了遗漏。

是一种结构化的方式。

{% endnote %}

### 更复杂的建模？

只使用结构体或者只使用枚举都会限制建模的能力，经常组合使用。例如计算不同**形状**的面积。在这个场景中，我们有一个抽象的概念“形状”，它下面有具体的变体，比如“圆形”和“矩形”。每种形状都有自己独特的属性（比如圆的半径，矩形的长和宽），但它们都共有一个行为：计算面积。

{% note primary %}

万事万物都是**状态机**！一个对象在任意时刻，只能处于几种预定义的状态之一（离散有限空间建模）。

- 在这样的建模问题中，类继承的方式选择抽取不同状态下的共同特性，并根据共同的特征设置抽象基类，并在此基础之上做多态性的继承。

    - 但是这样虽然提供了更高的抽象层级，但是不同类之间耦合程度高。（比如你在实现派生类的时候需要关注基类的数据成员和方法），同时可能会导致状态的遗漏。（因为C++的多态是运行时多态，因此会导致运行时报错）

- Rust 的哲学**倾向于组合而非继承的思路**，即**代数数据类型**。枚举负责表示互斥的状态标签。结构体负责承载每个状态特有的数据。 
    - 这样可以保证不同状态之间的独立性，缺点是需要手动维护（对于每一次新状态的加入）

{% endnote %}

#### 类继承实现

在 C++ 中，这种“**is-a**”关系（一个圆形**是一个**形状）非常适合用类继承来建模。我们定义一个抽象基类 `Shape`，它有一个纯虚函数 `area()`。然后，`Circle` 和 `Rectangle` 作为子类继承 `Shape`，并实现各自的 `area()` 函数。

```cpp
#include <iostream>
#include <vector>
#include <memory> // For std::unique_ptr

// 抽象基类
class Shape {
public:
    // 纯虚函数，子类必须实现
    virtual double area() const = 0;
    // 虚析构函数，防止内存泄漏
    virtual ~Shape() {}
};

// 子类：圆形
class Circle : public Shape {
private:
    double radius;
public:
    Circle(double r) : radius(r) {}
    double area() const override {
        return 3.14159 * radius * radius;
    }
};

// 子类：矩形
class Rectangle : public Shape {
private:
    double width, height;
public:
    Rectangle(double w, double h) : width(w), height(h) {}
    double area() const override {
        return width * height;
    }
};

// 处理不同形状的函数
void print_area(const Shape& s) {
    std::cout << "面积是: " << s.area() << std::endl;
}

int main() {
    // 使用多态性，创建指向基类的指针/引用，但实际对象是子类
    std::unique_ptr<Shape> my_circle = std::make_unique<Circle>(5.0);
    std::unique_ptr<Shape> my_rectangle = std::make_unique<Rectangle>(4.0, 6.0);
    
    print_area(*my_circle);
    print_area(*my_rectangle);

    return 0;
}
```


  * **运行时多态：** 函数 `print_area` 在运行时才知道它具体处理的是 `Circle` 还是 `Rectangle`，并调用相应的 `area()` 方法。
  * **开放扩展：** 如果要增加一个新形状，比如 `Triangle`，你只需要创建一个新的子类并实现 `area()` 函数，而不需要修改 `Shape` 基类或 `print_area` 函数。


#### What about in Rust？

在 Rust 中，我们使用**枚举**来代表“形状”，因为一个形状**要么是**圆形，**要么是**矩形。我们使用**结构体**来为每个枚举变体携带具体的数据。

```rust
// 使用结构体来代表具体形状的数据
struct CircleData {
    radius: f64,
}

struct RectangleData {
    width: f64,
    height: f64,
}

// 使用枚举来表示不同的形状变体，每个变体都包含相应的数据结构
enum Shape {
    Circle(CircleData),
    Rectangle(RectangleData),
}

// 为枚举实现方法
impl Shape {
    // 这是一个方法，可以在任何一个 Shape 实例上调用
    fn area(&self) -> f64 {
        // 使用 match 表达式进行模式匹配
        match self {
            Shape::Circle(data) => 3.14159 * data.radius * data.radius,
            Shape::Rectangle(data) => data.width * data.height,
        }
    }
}

// 处理不同形状的函数
fn print_area(s: &Shape) {
    println!("面积是: {}", s.area());
}

fn main() {
    let my_circle = Shape::Circle(CircleData { radius: 5.0 });
    let my_rectangle = Shape::Rectangle(RectangleData { width: 4.0, height: 6.0 });
    
    print_area(&my_circle);
    print_area(&my_rectangle);
}
```

**Rust 这种实现方式的特点：**

  * **编译时多态（基于 `match`）：** 编译器在编译时就知道 `Shape` 有 `Circle` 和 `Rectangle` 两个变体，`area()` 方法必须处理所有这些情况。
  * **封闭扩展：** 如果你要增加一个 `Triangle` 形状，你**必须**修改 `Shape` 枚举来添加 `Triangle` 变体，并且编译器会强制你在 `area()` 方法的 `match` 表达式中添加对 `Triangle` 的处理。这避免了遗漏情况。


```rust
// using the abstract data class
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage {
    write_info: String,
}

struct ChangeColorMessage(i32, i32, i32);

enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}
```

### Option

Option 枚举是标准库定义的枚举，实现了功能：一个值要么有值要么没值。

> Null is a value meaning no value.
> My billion-dollar mistakes

实际上，空指针的存在也是绝大多数编程语言报错崩溃的元凶。（或者说，很多 Bug 最终导致程序崩溃的最后一步就是不正确的操作最终导致了空指针的非法操作）。因此，**空还是非空**？这个问题，Rust从根源上杜绝了 null references 的出现。

```rust
// in standard library
pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}
```

```rust
let message_test = Message::Write(WriteMessage {
    write_info: String::from("test"),
});

let some_number = Some("5");
let absent_number: Option<i32> = None;
```

这样可以减少代码中无效的空值检查，并且巧妙的避免了空数据的操作问题（例如，对一个空的变量和另一个变量相加）

### 控制流

> 控制流永远不可能只停留在 if else

对于枚举变量的每一种可能，可以使用控制流 match 实现状态匹配，并对不同的变体实现不同的行为。

> 结构体 & 枚举的设计有一个致命的缺陷就是**代码的可维护性差**，对于枚举变量的修改（例如增加一个变体），需要手动修改每一处代码以及每一个 match 块。
> 不过这也是一种 trade off，即 fail fast，把错误暴露在编译时而不是运行时，当然，使用**trait**也是解决这个问题的方法之一。

```rust
enum Direction {
    North,
    East,
    South,
    West,
}

fn print_direction(dir: Direction) {
    // every possible state MUST be implemented
    match dir {
        Direction::North => println!("你正向北行驶"),
        Direction::East => println!("你正向东行驶"),
        Direction::South => println!("你正向南行驶"),
        Direction::West => println!("你正向西行驶"),
    }
}

fn main() {
    println!("Hello world!");
    print_direction(Direction::East);
}
```

### Advanced

- `if let`: syntactic sugar for match
- 解构相关变量（可以用在后续的判断和计算中），if let 也提供了解构的支持
- 哨兵模式：if 守卫提供进一步的检查
- 通配符来兜底 

```rust
// 定义 PlayerAction 事件的子类型
enum ActionType {
    Attack,
    UseItem,
}

// 定义游戏事件
enum Event {
    Quit,
    Move { x: i32, y: i32, speed: f32 },
    PlayerAction(u32, ActionType),
    NetworkData(String, Vec<u8>),
}

fn handle_event(event: Event) {
    match event {
        // 匹配并解构 Move 事件，将 x, y, speed 绑定到新变量
        Event::Move { x, y, speed } => {
            println!("玩家移动到 ({}, {})，速度：{}", x, y, speed);
        }

        // 匹配并解构 PlayerAction 事件，使用 (player_id, action) 模式
        // 在此处，我们进一步使用 if 守卫来匹配特定的动作类型
        Event::PlayerAction(player_id, action) => {
            // if let 是 match 的简化情况，适用于二元关系的判断（语法糖）
            if let ActionType::Attack = action {
                println!("玩家 {} 发动了攻击！", player_id);
            } else {
                println!("玩家 {} 执行了其他动作。", player_id);
            }
        }

        // 匹配并解构 NetworkData，同时使用 if 守卫来检查字符串
        Event::NetworkData(ref protocol_name, ref data) if protocol_name == "PING" => {
            println!("收到 PING 协议数据，长度：{}", data.len());
        }

        // 匹配并解构 NetworkData，但不对协议名做任何检查
        Event::NetworkData(protocol_name, data) => {
            println!("收到 {} 协议数据，数据长度：{}", protocol_name, data.len());
        }

        // 匹配剩下的所有情况
        Event::Quit => {
            println!("游戏退出事件被触发。");
        }

        _ => {
            println!("Something Error Occur");
        }
    }
}

fn main() {
    let event1 = Event::Move {
        x: 10,
        y: 20,
        speed: 5.5,
    };
    let event2 = Event::PlayerAction(123, ActionType::Attack);
    let event3 = Event::PlayerAction(456, ActionType::UseItem);
    let event4 = Event::NetworkData("PING".to_string(), vec![1, 2, 3]);
    let event5 = Event::NetworkData("HEARTBEAT".to_string(), vec![4, 5]);

    handle_event(event1);
    handle_event(event2);
    handle_event(event3);
    handle_event(event4);
    handle_event(event5);
}
```

### let...else

**`let else`** 是 Rust 2021 版中引入的一个语法糖（syntactic sugar），它为 **`if let`** 语句提供了一种更简洁、更符合人体工程学的方式，用于处理 Happy Path 和 Sad Path。

它最常用于**解构一个可能失败的值，并在失败时提前退出函数**。


`let else` 语句的语法如下：

```rust
let Some(value) = result else {
    // Sad Path (if result is None)
    // 这里的代码必须立即返回、panic 或 continue/break
    return;
};
// Happy Path (if result is Some)
// 这里的代码可以使用解构出的 value
println!("解构成功，值是：{}", value);
```

它的核心思想是：**“如果模式匹配成功，就把值绑定到变量上；否则，就执行 `else` 块中的代码。”**

#### `if let` vs. `let else`

要理解 `let else` 的优势，最好是和传统的 `if let` 进行对比。

使用 `if let` 处理 Happy Path 和 Sad Path 时，代码通常会嵌套一层，这在处理多个 `Option` 或 `Result` 时会变得很深。

```rust
// 假设这是函数体
let my_option = Some(10);
if let Some(x) = my_option {
    // Happy Path: 在 if 块内部处理
    // ... 大量代码 ...
    let my_second_option = Some(x + 5);
    if let Some(y) = my_second_option {
        // ... 更深一层的嵌套 ...
        println!("最终结果：{}", y);
    } else {
        // Sad Path 2
        return;
    }
} else {
    // Sad Path 1
    return;
}
```

这种写法随着解构的嵌套，会导致代码缩进越来越深，可读性变差，这就是所谓的“金字塔噩梦”（pyramid of doom）。

#### 使用 `let else` 的写法

`let else` 专门解决这个问题，**它将 Sad Path 提前处理，让 Happy Path 的代码保持在顶层，消除了嵌套**。

```rust
// 假设这是函数体
let my_option = Some(10);
let Some(x) = my_option else {
    // Sad Path 1: 失败时直接返回
    return;
};
// Happy Path: 代码继续向下流动，不再嵌套
// ... 大量代码 ...
let my_second_option = Some(x + 5);
let Some(y) = my_second_option else {
    // Sad Path 2: 失败时直接返回
    return;
};
// Happy Path: 代码继续向右流动，保持在顶层
println!("最终结果：{}", y);
```

“卫兵模式”（Guard Clause）是一种编程模式，它通过在函数开头放置多个条件检查，来处理非正常情况，从而让函数的主要逻辑保持清晰。

`let else` 是 Rust 中实现卫兵模式的完美工具：

  * **模式匹配作为条件**：`let else` 用模式匹配来代替简单的布尔判断。
  * **提前退出**：`else` 块中的代码必须是“非发散”的，即它必须通过 `return`、`panic!`、`continue` 或 `break` 来终止当前的控制流。这保证了 Sad Path 总是立即退出，不会继续执行后续代码。

这种设计使得代码结构更扁平，逻辑更直观，大大提高了可维护性。`let else` 让你能迅速过滤掉无效的输入，专注于处理程序的核心逻辑。
