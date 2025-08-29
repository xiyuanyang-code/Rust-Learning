# Generics and Traits in Rust

泛型是一个在几乎每一个编程语言中都有的概念，在 Rust 中也不例外。使用泛型能够提升代码的重用性，同时提供更高一层的抽象和泛化性。Rust 中的泛型可以广泛应用在函数，类和结构体中。与此同时， trait 定义泛型⾏为的⽅法。trait 可以与泛型结合来将泛型限制为**只接受拥有特定⾏为的类型，⽽不是任意类型**。最后介绍 ⽣命周期（lifetimes）：⼀类允许我们向编译器提供引⽤如何相互关联的泛型。Rust 的⽣命周期功能允许在更多场景下借⽤值的同时仍然使编译器能够检查这些引⽤的有效性⽽不⽤借助我们的帮助。

## Generics

我们来看下面的例子，我们实现了一个**寻找数列中最大值**的基本工具函数：

```rust
fn main() {
    println!("Hello world!");
    let test = vec![1,23,5,6,];
    let result = find_max_value(&test).unwrap();
    println!("{}", result);

    let empty_test: Vec<i32> = Vec::new();
    let result = find_max_value(&empty_test);
    println!("{result:?}");
}

fn find_max_value(numbers: &Vec<i32>) -> Result<&i32, String> {
    if numbers.is_empty() {
        return Err("The input vector cannot be empty.".to_string());
    }

    let mut max_num = &numbers[0];

    for num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }

    Ok(max_num)
}
```

上述的函数签名规定了输入数据类型必须为 `&Vec<i32>`，但是实际上只要满足以下条件，就理论上都有对应的函数实现：

- 传入对象是**可迭代的**。

- 具体迭代的数据类型必须重载了相关的比较运算符，即两个值之间可以进行比较大小。

我们可以使用**泛型**来实现上述要求，先展示结果：

```rust
fn main() {
    // using generics
    let numbers_vec = vec![1, 5, 2, 8, 3];
    let numbers_slice = [10, 5, 20, 15];
    let empty_vec: Vec<i32> = vec![];

    // 使用 Vec
    if let Some(max) = find_max_value_gen(&numbers_vec) {
        println!("The max value in vec is: {}", max); // Output: The max value in vec is: 8
    }

    // 使用 slice
    if let Some(max) = find_max_value_gen(&numbers_slice) {
        println!("The max value in slice is: {}", max); // Output: The max value in slice is: 20
    }

    // 处理空输入
    if let None = find_max_value_gen(&empty_vec) {
        println!("The input collection is empty."); // Output: The input collection is empty.
    }
}

fn find_max_value_gen<'a, I, T>(numbers: I) -> Option<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    T: Ord + 'a,
{
    let mut numbers_iter = numbers.into_iter();

    if let Some(mut max_num) = numbers_iter.next() {
        for num in numbers_iter {
            if num > max_num {
                max_num = num;
            }
        }
        Some(max_num)
    } else {
        None
    }
}

// The max value in vec is: 8
// The max value in slice is: 20
// The input collection is empty.
```

### 在函数定义中使用泛型

我们不妨先简化一点，实现第二点的小要求：即仍然使用 Vector 作为固定的数据类型对象，但是 Vector 内部的类型值实现泛型。

```rust
fn find_max_for_vec<T: Ord>(numbers: &Vec<T>) -> Option<&T> {
    // Ord is a trait, ensuring generic T must can be compared
    if numbers.is_empty() {
        return None;
    }

    let mut max_num = &numbers[0];

    for num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }

    Some(max_num)
}
```

可以看到，函数内部的实现逻辑几乎没有任何变化（这也证明了这个函数具有泛化的能力，不依赖于具体的类型），唯一变化的就是**函数签名**的部分。

```rust
fn find_max_for_vec<T: Ord>(numbers: &Vec<T>) -> Option<&T> {
    // Ord is a trait, ensuring generic T must can be compared
    if numbers.is_empty() {
        return None;
    }

    let mut max_num = &numbers[0];

    for num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }

    Some(max_num)
}

fn find_max_for_list<T: Ord>(numbers: &[T]) -> Option<&T> {
    if numbers.is_empty() {
        return None;
    }

    let mut max_num = &numbers[0];

    for num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }

    Some(max_num)
}
```

但是我们会发现我们仍然需要为不同的可迭代对象创建不同的函数（哪怕这些函数一定对数据类型层面做了泛型），这些更加高级的功能需要使用到 Trait 和生命周期的概念，具体的讲解见下文。现在，我们先将目光转移到结构体等数据结构的泛型机制上。

### 结构体定义中的泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

struct Point_tuple<T>(T, T);

let struct_test_1 = Point{
    x: 100,
    y: 100,
};

let struct_test_2 = Point_tuple(100, 2000);
```

和 C++ 一样，编译器会自动推断具体的数据类型，并在推断失败的时候编译报错。

### 枚举定义 & 方法定义的泛型

```rust
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T>{
    fn get_x(&self) -> &T{
        &self.x
    }
}

impl Point<i32>{
    fn spec_for_i32(&self) -> &i32{
        println!("It is specially designed for i32 type!");
        &self.x
    }
}
```

当然，你也可以使用更多的泛型：

```rust
// ! WARNING: This will cause an error
impl<T> Point<T> {
    fn mixed_point<X>(&self, other: &Point<X>) -> (&T, &X) {
        (&self.x, &other.y)
    }
}
```

上文的函数又引入了一个新的泛型 X，但是同时引入了新的逻辑漏洞，看似完美的执行逻辑实则可能会因为**生命周期的不同引发 Bug**。它返回的元组中包含了来自两个不同 `Point` 实例的引用，但它们的生命周期可能不一致。

1.  **`&self.x` 的生命周期**: 这个引用 (`&T`) 的生命周期与 `self` 相同。也就是说，它的生命周期至少和调用 `mixed_point` 函数的 `Point` 实例一样长。
2.  **`&other.y` 的生命周期**: 这个引用 (`&X`) 的生命周期与 `other` 相同。它的生命周期可能与 `self` 不同，甚至更短。
3.  **返回类型 `(&T, &X)`**: Rust 的编译器无法知道这两个引用的生命周期关系。当它们被组合在一个元组中返回时，编译器会发出一个错误，因为它无法保证**这个元组中的两个引用在函数调用结束后仍然有效**。

例如下面的代码，就会出现问题：

```rust
fn main() {
    let p1 = Point { x: 5, y: 10 }; // p1 的生命周期到 main 函数结束
    let result;

    { // 这是一个内部作用域
        let p2 = Point { x: "hello", y: "world" }; // p2 的生命周期只在这个作用域内
        result = p1.mixed_point(&p2); // 尝试调用 mixed_point
    } // p2 在这里被销毁，其引用的数据也随之消失

    // 尝试在 p2 已经被销毁后使用 result
    println!("x: {}, y: {}", result.0, result.1); 
}
```

对于上面的问题，有两种解决办法：

- 直接创建新的元组（因为是栈上元素所以直接复制），这样就不会存在生命周期不一致的问题。

- 在泛型函数中加入生命周期的控制。
    - 让编译器明确每一个传入变量的生命周期，保证后续调用这些函数的安全性。

```rust
impl <T> Point<T>{
    fn mixed_point<X>(self, other: Point<X>) -> (T, X){
        (self.x, other.y)
    }
}

impl <T> Point<T> {
    fn mixed_point_life<'a, 'b, X>(&'a self, other: &'b Point<X>) -> (&'a T, &'b X) {
        (&self.x, &other.y)
    }
}
```

> Rust 在运行泛型代码的时候会**执行代码的单态化**来保证运行效率。具体来说，编译器寻找所有泛型代码被调⽤的位置并使⽤泛型代码针对具体类型⽣成代码。因此泛型不会在程序运行时引入性能损失。

## Trait

Trait 有点类似于抽象基类的功能，定义共享行为的语言特性(契约)，它规定了某种类型必须具备哪些方法。

```rust
pub trait Summary{
    fn summarize(&self) -> String;
    // self 代表实现该类型的实例的 self
}
```

在定义了这个 trait 之后，我们就可以为特定的类型（结构体）实现对应的 trait，之后就可以像成员函数一样访问：

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    println!("Hello world!");
    let news_1 = NewsArticle {
        headline: String::from("It is a headline"),
        location: String::from("China"),
        author: String::from("Xiyuan Yang"),
        content: String::from("It is a content"),
    };
    println!("{}", news_1.summarize());
}
```

如果对于某个类型的实现为空，这会调用 Trait 定义中的默认方法。

```rust
pub struct EmptyPost {
    pub username: String,
}

// empty trait
// using the default method
impl Summary for EmptyPost {}
```

默认实现允许调用实现中的其他方法，哪怕这些方法没有默认实现。

> 简单来说，如果 trait 中的一个方法已经实现了默认方法，那么在具体的类型中可以选择自定义来 override 这个方法，也可以选择不实现，使用默认方法。
> 但是如果 trait 中的方法并没有提供默认实现，那么根据必须实现，否则编译器会报错。

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        format!("READ more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // 这里我们只实现了 summarize_author
    // summarize 会使用 trait 的默认实现
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    // 这里我们只实现了 summarize_author
    // summarize 会使用 trait 的默认实现
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.username)
    }
}

pub struct EmptyPost {
    pub username: String,
}

// 这里我们只实现了 summarize_author
// summarize 会使用 trait 的默认实现
impl Summary for EmptyPost {
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.username)
    }
}

fn main() {
    println!("Hello world!");

    let news_1 = NewsArticle {
        headline: String::from("It is a headline"),
        location: String::from("China"),
        author: String::from("Xiyuan Yang"),
        content: String::from("It is a content"),
    };
    println!("{}", news_1.summarize()); // 输出: READ more from Author: Xiyuan Yang...

    let empty_news = EmptyPost {
        username: String::from("It is an empty post"),
    };
    println!("{}", empty_news.summarize()); // 输出: READ more from Author: It is an empty post...
}
```

### Trait 作为参数

Trait 也可以作为函数参数被传入，对应的类型注解是 `&impl`.

```rust
pub fn notify(item: &impl Summary){
    println!("Breaking News! {}", item.summarize());
}
```

也可以在返回值中使⽤ impl Trait 语法，来返回实现了某个 trait 的类型。

### Trait 与 泛型

Trait 和泛型的关系就在于 **Trait** 作为参数被传递的时候本质就是一种**泛型写法**的语法糖。例如，下面的两个函数签名是等价的。

```rust
pub fn notify_new(item_1: &impl Summary, item_2: &impl Summary){}
pub fn notify_new_2<T: Summary>(item_1: &T, item_2: &T){}
```

这就是 Trait 和泛型的联系！通过将 Trait 作为函数参数传递，实则也约束了被传入的实例的借用必须要实现对应的 Trait。

```rust
pub fn notify_double(item: &(impl Summary + Display)){}
pub fn notify_double_2<T: Summary + Display>(item: &T){}
```

{% note primary %}

当函数签名比较简单的时候，使用语法糖就可以很清晰的显示具体约束的 Trait 有哪些。但是如果不同的参数有对应不同的 Trait 的约束，还是得回到泛型的定义中去。

{% endnote %}

我们可以使用 where 从句来简化这个泛型，使其更加的可读：

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}
```

## 生命周期

Rust 中的每一个引用都是存在生命周期的，并且 Rust 的编译器聪明的把每一个引用的生命周期都压缩到了理论最短。只要保证在任意时刻内**仍然在生命周期中的引用**保持所有权借用的规则，编译器就会认为代码是安全的。

与此同时，借用还需要保证：**借用的生命周期**不可以超过**原来数据的生命周期**，否则就会产生悬垂引用的严重错误。

### 泛型生命周期

```rust
//! bad code
fn find_longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}
```

上述代码无法通过编译：

```text
error[E0106]: missing lifetime specifier
 --> src/bin/lifetime.rs:1:38
  |
1 | fn find_longest(a: &str, b: &str) -> &str {
  |                    ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
1 | fn find_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
  |                ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `rust-learning` (bin "lifetime") due to 1 previous error
```

为什么出现错误？因为 Rust 无法确定**返回值的生命周期**，实际上，谁都无法确定（因为这个由传入的函数参数所决定）。Rust 必须要保证其 Borrow Checker 明确每一个借用的生命周期，这是保证安全性的基础。

因此，为了让 Borrow Checker 不再困惑，我们需要**显示添加生命周期的注解**。

```rust
&i32 // 引⽤
&'a i32 // 带有显式⽣命周期的引⽤
&'a mut i32 // 带有显式⽣命周期的可变引⽤
```

因此，我们可以添加一些类型注解，这样就可以通过编译了：

```rust
fn find_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

* **`<'a>`**：这是一个**生命周期参数声明**，它告诉编译器，我们将在函数签名中使用一个名为 `'a` 的生命周期。
* **`a: &'a str`** 和 **`b: &'a str`**：这表示参数 `a` 和 `b` 的生命周期至少要像 `'a` 一样长。
* **`-> &'a str`**：这表示返回值的生命周期也和 `'a` 一样长。

通过这种方式，你创建了一个**生命周期约束**：函数返回的引用的生命周期，和它所有输入引用的生命周期中**较短**的那一个保持一致。例如，如果 `a` 和 `b` 的生命周期分别是 $L_a$ 和 $L_b$，那么 `'a` 的具体生命周期将是 $\min(L_a, L_b)$。

不过这并不代表函数在使用时就不会引发编译错误！生命周期的注解只是额外给编译器提供更多的信息，让编译器在一些存在安全隐患的情况下及时发出警报。

```rust
fn main() {
    // PASSED
    println!("Hello world!");
    let long_string = String::from("hello world");
    let result ;
    {
        let short_string = String::from("test");
        result = find_longest(&long_string, &short_string);
        println!("The result is {}", result);
    }
    // after the scope, result is never used
    // println!("The result is {}", result);
}
```

```rust
fn main() {
    // FAILED
    println!("Hello world!");
    let long_string = String::from("hello world");
    let result ;
    {
        let short_string = String::from("test");
        result = find_longest(&long_string, &short_string);
        println!("The result is {}", result);
    }
    // after the scope, result is never used
    println!("The result is {}", result);
}
```

这实际上是一个很隐蔽的错误，如果这个函数体更加复杂，设计的作用域嵌套更加多的时候，因此使用编译器来检查并杜绝这种安全隐患是及其有必要的。

> 如果函数返回的是一个引用，那么其生命周期应该至少和一个参数的生命周期绑定，如果不是的话，那么就说明这个返回值是内部生成的（所有权在内部），那在这个时候函数返回值，所有权在函数内部被清理，产生了悬垂引用。

### 结构体定义中的生命周期

在创建包含引用的结构体时，也需要关注生命周期的问题（因为所有权不在函数生命周期的内部）

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

对应的，也可以对对应方法的实现添加相关的生命周期注解：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

### 静态生命周期

使用 `static` 声明变量可以让对应的变量存在于**整个生命周期中**。

{% note primary %}

注意！Rust 中没有**全局变量**，为了保证安全性。

{% endnote %}

```rust
let s: &'static str = "I have a static lifetime.";
```

## CallBack

最后，我们再来看之前的**找可迭代对象中的最大值**的函数，在学习了上述的有关 Trait 和 生命周期的相关知识后，我们也可以实现最终最高抽象程度的泛型：

```rust
fn find_max_value_gen<'a, I, T>(numbers: I) -> Option<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    T: Ord + 'a,
{
    let mut numbers_iter = numbers.into_iter();

    if let Some(mut max_num) = numbers_iter.next() {
        for num in numbers_iter {
            if num > max_num {
                max_num = num;
            }
        }
        Some(max_num)
    } else {
        None
    }
}
```

- **`< 'a, I, T >`**: 这是一个泛型参数列表。
    - **`'a`**: 这是一个**生命周期参数**。它确保了函数的返回值（`&'a T`）的生命周期，与输入参数 `numbers` 中元素的引用生命周期一样长。这意味着返回的引用不会在原始数据被销毁后仍然存在，避免了悬空引用。
    - **`I`**: 这是一个**泛型类型参数**，代表了输入参数的类型。
    - **`T`**: 这是一个**泛型类型参数**，代表了集合中元素的类型。
- **`(numbers: I)`**: 这是函数的输入参数。`numbers` 是一个类型为 `I` 的变量。
- **`-> Option<&'a T>`**: 这是函数的返回值。它返回一个 `Option` 枚举，这是一种常见的 Rust 模式，用于处理可能失败的操作。
    - **`Some(&'a T)`**: 如果找到了最大值，就返回一个包含最大值引用的 `Some` 变体。
    - **`None`**: 如果输入的集合为空，则返回 `None`。
- **`where` 子句**: 这部分对泛型类型 `I` 和 `T` 进行了约束，确保它们具有函数所需的功能。
    - **`I: IntoIterator<Item = &'a T>`**: 要求 `I` 必须是一个可以转换为 **迭代器** (`IntoIterator`) 的类型。这个迭代器产生的每个元素 (`Item`) 都必须是类型为 `T` 的引用，且生命周期为 `'a`。
    - **`T: Ord + 'a`**: 要求类型 `T` 必须实现了 `Ord` trait。`Ord` trait 提供了比较大小的功能，如 `>`、`<` 等。`+ 'a` 约束表示 `T` 类型本身不能包含比 `'a` 短的生命周期，这通常是编译器为了安全自动推导出来的。
