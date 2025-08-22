# Rust Ownership

## Introduction

## 所有权机制

- 一些程序语言的垃圾收集器会自动收集不再使用的内存（Java, Python），这样的垃圾回收机制会带来性能上的额外开销（例如Stop The World，在这个阶段所有应用程序线程的执行都会被暂停）
- 一些程序语言需要程序员**手动进行内存管理**。（C++的手动 new 和 delete）

Rust使用**所有权系统**来管理内存，并且不会带来额外的运行开销。（因为内存的检查和管理在编译时就已经完成）

## 栈内存和堆内存

- 栈内存（栈帧空间），LIFO
	- **所有存储在栈上的数据必须具有已知并且固定的大小**。
- 堆内存
	- 在堆上进行内存分配（寻找可用的空间并且标记指针，这个指针可以存储在栈上）
	- 访问堆内存会更加慢（因为需要**指针间接跳转访问**）

在C++中，我们经常在堆内存中为程序分配内存，并且需要手动删除，因此**所有权需要解决的问题主要就集中在堆内存的分配和管理上**，即跟踪代码的哪些部分使用了 heap 的哪些数据，并且需要清理 heap 中的数据来避免空间不足的问题。

> 即进行垃圾回收的机制（但是是在编译时执行）

## 所有权规则

{% note primary %}

- 每个值都有一个变量，该变量是该值的所有者。
- 每个值**同时只可以拥有一个所有者**。
- **当所有者超出作用域(Rust)的时候，该值将会被删除**。
    - 对于第三点，当所有者超出自身作用域的时候，**内存会立即被交换给操作系统**。（这一点和C++不同）

{% endnote %}

### String in Rust

字符串字面值是不可变的，因为变量的值在编译时确定，因此，Rust 专门设计了可变的 String 类。对于一个String类的变量，程序会在内存中储存三个信息：`ptr`, `length` and `capacity`，这三个信息本身储存在栈内存中。而指针所指向的具体的内存（就是实际的data）是分配在堆内存中的。

使用 `String::from` 的时候，他会在运行时向内存分配器请求分配内存（这一点与字符串字面值不同）。当变量离开作用域之后，会自动调用 `drop` 函数来自动释放内存（类似于 C++ 中的资源获取即初始化，RAII）。 

### 深拷贝、浅拷贝和移动语义

在 C++ 中，存在深拷贝和浅拷贝的区别，简单来说，任何包含指向**动态分配内存的指针**的类型，如果不对其进行特殊处理（即不自定义拷贝构造函数和拷贝赋值运算符），那么它就会执行默认的浅拷贝，即只拷贝指针变量的值（地址），而并不是对指针所指向的变量做复制。

- 对于 Rust 来说，**浅拷贝**是绝对不允许的。实际上在 C++ 中，浅拷贝现象也会导致悬空指针或者二次 delete 等问题，是内存泄漏的重要原因之一。Rust的核心宗旨之一就是**物皆有主**：每一个值同时只可以拥有一个所有者。

- 深拷贝本身更加的安全，但是会额外分配资源，因此如果你需要创建一个独立的、在堆上拥有自己数据的副本，必须显式地调用 `.clone()` 方法。

> 在 Rust 中，所有权规则有一个例外：对于一些**存储在栈上的、大小固定的简单类型**，它们在赋值时会发生复制，而不是所有权转移。这些类型实现了 Copy trait，这意味着它们在赋值时，会直接复制底层的值，而不是移动所有权。

在现代 C++ 中引入了**移动语义**的概念，即在避免复制的情况下完全移交所有权给对应变量，这样高效地解决了同一块内存被两个指针同时指向（浅拷贝带来的问题）的隐患。Rust 于是借用了这样的机制，这也是 Rust 对储存在堆内存上变量的核心机制之一：**移动**。

- 当你将一个变量（比如一个 `String`）赋值给另一个变量时，Rust 默认不会像 C++ 那样进行浅拷贝，而是会将这个变量的所有权转移给新变量。例如下面的代码，在 s2 的作用域上，`let s2 = s1`，`s1` 变量的值的所有权发生了转移，此时再次调用 s1 就会报错，因为 s1 已经不再有对变量的所有权。

> 从安全性的角度考虑，Rust 希望值和变量是严格绑定的，杜绝浅拷贝现象的发生。而移动语义的机制也实现了这一点，可以让开发者迅速的定位到哪里有着非法的操作，而不是等到程序写完了出现 Segmentation Fault。

```rust
fn main() {
    let s1 = String::from("Hello world");
    println!("s1: {}", s1);
    {
        println!("Entering the scope of s2");
        let s2 = s1;
        println!("s2: {}", s2);
    }

    println!("{}", s1);
}
```
### Clone

可否进行补救？当然可以，Rust提供了 `.clone()` 函数来实现深拷贝。

```rust
fn main() {
    let s1 = String::from("Hello world");
    println!("s1: {}", s1);
    {
        println!("Entering the scope of s2");
        let s2 = s1.clone();
        println!("s2: {}", s2);
    }

    println!("{}", s1);
}
```

{% note primary %}

在 Rust 中，所有权规则有一个例外：对于一些**存储在栈上的、大小固定的简单类型**，它们在赋值时会发生复制，而不是所有权转移。这些类型实现了 Copy trait，这意味着它们在赋值时，会直接复制底层的值，而不是移动所有权。

例如下面的程序可以正常执行：

```rust
fn main() {
    let x = 5;
    {
        let y = x;
        println!("y = {}", y);
    } 

    println!("x = {}", x);
}
```

{% endnote %}


{% note primary %}

### `Copy` Trait

在 Rust 中，**`Copy` Trait** 是一个核心概念，它决定了类型在赋值时是执行**移动（Move）还是复制（Copy）**。

`Copy` Trait 标志着一个类型可以被简单地复制，而不会发生所有权转移。当一个类型实现了 `Copy` Trait，它的变量在赋值或作为函数参数传递时，会直接在栈上进行**按位复制**，而不是所有权移动。

这通常适用于那些**存储在栈上、大小固定且不拥有任何堆上资源**的类型。理论上，**只有所有成员都储存在栈上的类型才能实现 Copy Trait**，而**不可以拥有需要 Drop Trait 管理的堆内存资源**。

> 因为 Copy Trait 是直接在栈内存上的快速按位复制，不涉及堆内存的操作，因此开销极低。
> 如果涉及堆内存，那在栈上会有一个指针存储指向堆内存的相关地址，而这个指针就会在 Copy Trait 中被复制，而堆内存不会被复制，又回到了浅拷贝出现的问题上。


你可以通过为你的 `struct` 或 `enum` 派生（derive）`Copy` Trait 来实现它。

**要实现 `Copy` Trait，一个类型必须满足两个条件：**

1.  **所有成员都必须实现了 `Copy` Trait。**
2.  **它不能拥有析构函数（`Drop` Trait）。** 因为如果一个类型实现了 `Drop`，它就需要在离开作用域时执行特定的清理操作。如果它能被复制，那么就有可能导致重复的清理，这会破坏内存安全。
> Drop 方法是用于自动释放堆内存的，栈内存的释放是根据 LIFO 规则自动释放的。

例如，下面这个 `Point` 结构体可以派生 `Copy` Trait，因为它的两个成员 `x` 和 `y` 都实现了 `Copy`：

```rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = p1; // 发生了复制
    
    println!("p1 = {:?}", p1); // p1 仍然有效
    println!("p2 = {:?}", p2);
}
```

相反，下面这个 `Container` 结构体就**不能**实现 `Copy` Trait，因为它包含了一个 `String` 成员，而 `String` 类型没有实现 `Copy`：

```rust
struct Container {
    id: i32,
    name: String,
}

fn main() {
    let c1 = Container { id: 1, name: String::from("Rust") };
    let c2 = c1; // 发生移动，c1 不再有效
    
    // println!("c1.id = {}", c1.id); // 编译错误！
    println!("c2.name = {}", c2.name);
}
```

#### `Copy` Trait 和 `Clone` Trait 

这两个 Trait 经常被一起讨论，但它们的用途不同：

  - **`Copy` Trait**：用于**隐式**的按位复制，开销极低。它发生在赋值和传参时，是 Rust 的默认行为之一。
  - **`Clone` Trait**：用于**显式**的深拷贝。当你需要创建一个独立的、拥有自己堆上数据副本时，必须调用 `.clone()` 方法。

一个类型只有实现了 `Clone` Trait 才能实现 `Copy` Trait，因为 `Copy` 是 `Clone` 的一个特殊情况。因此，在派生 `Copy` 时，通常也会派生 `Clone`。

{% endnote %}

### 所有权与函数

本质上和变量赋值完全类似，**可以进行移动或者复制**。

- 对于Copy Trait，函数的传参就是复制的过程，不会影响原变量的内容。
- 但是对于非 Copy Trait，就相当于C++中的&&右值引用，直接交出了变量的所有权。

函数的返回值也是如此。返回值也可以发生所有权的转移。

## 引用和借用

{% note info %}

- 要么全是不可变引用，要么只能有一个可变引用

- 引用的生命周期不可以超过数据的生命周期

{% endnote %}

Rust所有权机制设计的初衷是**保证内存安全**，但是这样的设计会带来非常大的麻烦，尤其是在面对函数的反复传参的过程中。我们来看下面的代码：

```rust
fn main() {
    // println!("Hello, world!");
    let s1 = String::from("Hello world");
    let length = cauculate_length(s1);

    println!("{}", length);
    println!("{}", s1);
}


fn cauculate_length(s: String) -> usize{
    s.len()
}
```

程序果不其然的报错了，因为在函数传参的时候，main函数作用域里的`s1`就已经被“销毁”了（或者说没有控制权了），而控制权被转移到了函数体内部的临时变量上，出函数之后离开作用域，这个字符串被释放。

我们可以有以下几种方式避免这一种错误：

首先就是使用`.clone()`函数再拷贝一份（但这会带来效率上的下降）

```rust
fn main() {
    // println!("Hello, world!");
    let s1 = String::from("Hello world");
    let length = cauculate_length(s1.clone());

    println!("{}", length);
    println!("{}", s1);
}


fn cauculate_length(s: String) -> usize{
    s.len()
}
```

同样，我们也可以修改**函数的参数和返回值列表**，让对应的临时变量返回所有权。但是这样会导致函数接口的变化，并且难以应对函数参数列表比较复杂且庞大的现象。

```rust
fn main() {
    // println!("Hello, world!");
    let s1 = String::from("Hello world");
    let (length, s1) = cauculate_length(s1);

    println!("{}", length);
    println!("{}", s1);
}

fn cauculate_length(s: String) -> (usize, String){
    (s.len(), s)
}
```

Rust还实现了第三种方式，即在函数内部（临时作用域）中进行**变量的借用**。（有点类似于C++中的reference）

### 引用

**Definition**：允许你取得某些值但不获得所有权。

```rust
fn main() {
    // println!("Hello, world!");
    let s1 = String::from("Hello world");
    let length = cauculate_length(&s1);

    println!("{}", length);
    println!("{}", s1);
}

fn cauculate_length(s: &String) -> usize{
    s.len()
}
```

{% note info %}

引用的本质就是一个**指针**！

![引用](https://s1.imagehub.cc/images/2025/04/19/595ceb3d5d105ec448dfbd9a809503c9.png)

{% endnote %}

这样就可以保证**临时变量在走出作用域的时候**，因为没有所有权所以内存不会被回收。

**但是我们不可以修改借用的东西！**（const T&）

### 可变引用

```rust
fn main() {
    // println!("Hello, world!");
    let mut s1 = String::from("Hello world");
    println!("{}", s1);
    let length = cauculate_length(&mut s1);
    println!("{}", length);
    println!("{}", s1);
}

fn cauculate_length(s: &mut String) -> usize {
    s.push_str("I am a boy");
    s.len()
}
```

- 使用`&mut`。
- **可变引用**有一个非常重要的性质：**在特定的作用域内**，对于同一块数据，只允许存在一个可变的引用。（防止数据竞争）
- 不过可以在不同的作用域内（不同时）

```rust
fn main() {
    // println!("Hello, world!");
    let mut s1 = String::from("Hello world");
    println!("{}", s1);
    let length = cauculate_length(&mut s1);
    println!("{}", length);
    println!("{}", s1);


    let mut s3 = String::from("Hello wor");
    let s4 =  & mut s3;
    let s5 = & mut s3;

    println!("{}{}", s4, s5);
    

}

fn cauculate_length(s: &mut String) -> usize {
    s.push_str("I am a boy");
    s.len()
}
```

报错：

```
error[E0499]: cannot borrow `s3` as mutable more than once at a time
  --> src/main.rs:22:14
   |
21 |     let s4 =  & mut s3;
   |               -------- first mutable borrow occurs here
22 |     let s5 = & mut s3;
   |              ^^^^^^^^ second mutable borrow occurs here
23 |
24 |     println!("{}{}", s4, s5);
   |                      -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `Borrow` (bin "Borrow") due to 1 previous error
```

- **不可以同时拥有一个可变引用和一个不可变引用**。（允许只存在多个不可变的引用或者唯一一个可变引用）

{% note primary %}

数据竞争（Data Race）是一种在并发编程中常见的、严重且难以调试的错误。它发生在多个线程或进程同时访问共享数据时，并且至少有一个访问是写入操作，而这些访问没有通过同步机制进行协调。

数据竞争的发生必须同时满足以下三个条件：

1.  **并发访问**：至少有两个或更多线程或进程在同时运行。
2.  **共享数据**：这些线程或进程访问了同一块共享内存区域。
3.  **至少一个写入**：至少有一个访问是写入操作（修改数据）。如果所有访问都是读取，则不会发生数据竞争。
4.  **缺乏同步**：这些访问操作没有使用互斥锁、信号量或其他同步原语来保证其原子性和有序性。

当数据竞争发生时，程序的行为变得**不可预测**，可能导致以下严重问题：

  * **结果不一致**：最终结果可能取决于线程执行的顺序。例如，两个线程同时对一个变量进行 `++` 操作，最终结果可能是 `+1` 而不是预期的 `+2`。
  * **程序崩溃**：一个线程可能正在读取数据，而另一个线程正在修改或释放同一块数据，导致读取到无效数据（野指针）并引发崩溃。
  * **死锁或活锁**：间接导致更复杂的同步问题。
  * **难以调试**：数据竞争是时序相关的错误，它可能只在特定的线程调度顺序下出现，而且很难复现。

Rust 的所有权机制从根源上杜绝了数据竞争现象的出现。

{% endnote %}


### 悬空引用 (Dangling References)

在C++中，存在Dangling Pointer的概念，即一个指针指向了内存中的某个有效地址，而该内存被释放之后指针本身的值（所指向的地址）并没有发生变化，但此时**指向了一块无效的内存**，产生严重的错误，并且这个错误不会被编译器检查到。

```cpp
#include <iostream>

int main() {
    int *ptr = new int(5);
    *ptr += 1;
    std::cout << ptr << std::endl;
    std::cout << *ptr << std::endl;
    delete ptr;

    std::cout << ptr << std::endl;
    std::cout << *ptr << std::endl;
    return 0;
}
```

程序输出：

```
0x55aba4ee42b0
6
0x55aba4ee42b0
1522159332
```

但是在**Rust**中，可以保证**不存在悬空引用**，因为**引用在离开作用域之前数据都不会离开作用域**。

```rust
fn main(){
    let v = dangle();
    println!("{}", v);
}

fn dangle() -> &String{
    let s = String::from("Hello");
    &s
}
```

在函数内部的变量 s 在函数返回时交出了自己的应用（注意思考应用的本质：**是一个指针**！），但是在函数结束时 s 会立刻调用 drop trait（离开作用域）而导致堆内存释放，因此，此时返回的应用**指向了一块无效的堆内存**。（就是 C++ 的悬垂指针）

这个问题是非常严重的，会打破 Rust 的安全性，因此，Rust 会在编译时杜绝这个行为，即**引用在离开作用域之前数据都不会离开作用域**。换句话说，引用的生命周期不能超过其所引用的数据的生命周期。（在这里我们需要把值的所有权移交）

```
  Compiling Borrow v0.1.0 (/home/xiyuanyang/Rust/Borrow)
error[E0106]: missing lifetime specifier
  --> src/main.rs:17:16
   |
17 | fn dangle() -> &String{
   |                ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
   |
17 | fn dangle() -> &'static String{
   |                 +++++++
help: instead, you are more likely to want to return an owned value
   |
17 - fn dangle() -> &String{
17 + fn dangle() -> String{
   |

error[E0515]: cannot return reference to local variable `s`
  --> src/main.rs:19:5
   |
19 |     &s
   |     ^^ returns a reference to data owned by the current function

Some errors have detailed explanations: E0106, E0515.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `Borrow` (bin "Borrow") due to 2 previous errors
```

## Slice

Slice 是一种特殊的引用，允许**引用集合中一段连续的元素序列**，而不是应用整个集合。方便高效的查看字符串的数据。

```rust
fn main() {
    println!("Hello world!");
    let s = String::from("hello world");
    println!("{}", first_word(&s));

    let s_1 = &s[1..=3]; // [1,3]
    let s_2 = &s[1..3];  // [1,3)
    println!("{s_1}");
    println!("{s_2}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
```

> Slice 是使用依然要遵守相关引用的规则。

这段程序会报错，因为发生了数据竞争，同时出现了可变引用和不可变引用。

```rust
fn main() {
    println!("Hello world!");
    let mut s = String::from("hello world");

    let mut test_slice = &mut s;
    
    let s_1 = &s[1..=3]; // [1,3]
    let s_2 = &s[1..3];  // [1,3)
    test_slice.push_str("what???");
    println!("{s_1}");
    println!("{s_2}");
}
```

但是这段程序不会，在创建这两个引用之前，test_slice 已经完成了它的任务，并且它的生命周期在理论上已经结束了。因此，当创建 s_1 和 s_2 这两个不可变引用时，可变引用已经不再活跃。

```rust
fn main() {
    println!("Hello world!");
    let mut s = String::from("hello world");

    let mut test_slice = &mut s;
    test_slice.push_str("what???");
    
    let s_1 = &s[1..=3]; // [1,3]
    let s_2 = &s[1..3];  // [1,3)
    println!("{s_1}");
    println!("{s_2}");
}
```

> 字符串的字面值就是 slice