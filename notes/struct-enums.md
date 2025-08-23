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

