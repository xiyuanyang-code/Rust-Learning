# Rust Ownership

## Introduction

## 所有权机制

- 一些程序语言的垃圾收集器会自动收集不再使用的内存（Java）
- 一些程序语言需要程序员**手动进行内存管理**。（C++的手动new和delete）

Rust使用**所有权系统**来管理内存，并且不会带来额外的运行开销。（因为内存的检查和管理在编译时就已经完成）

## 栈内存和堆内存

- 栈内存（栈帧空间），LIFO
	- **所有存储在栈上的数据必须具有已知并且固定的大小**。
- 堆内存
	- 在堆上进行内存分配（寻找可用的空间并且标记指针）
	- 访问堆内存会更加慢（因为需要**指针间接跳转访问**）

在C++中，我们经常在堆内存中为程序分配内存，并且需要手动删除，因此**所有权需要解决的问题主要就集中在堆内存的分配和管理上**，即跟踪代码的哪些部分使用了heap的哪些数据，并且需要清理heap中的数据来避免空间不足的问题。

## 所有权规则

{% note primary %}

- 每个值都有一个变量，该变量是该值的所有者。
- 每个值**同时只可以拥有一个所有者**。
- **当所有者超出作用域(Rust)的时候，该值将会被删除**。

{% endnote %}

对于第三点，当所有者超出自身作用域的时候，内存会立即被交换给操作系统。（这一点和C++不同）

这真的是非常严格的所有权规则！我们来看下面的一个例子：

> 前提：**Rust**中的String类会被存储在**堆内存中**，因为其可变性，而不是在栈内存中。

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

在这里执行第10行代码会产生比较严重的编译错误，输出如下：

```
Compiling ownership v0.1.0 (/home/xiyuanyang/Rust/ownership)
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:21:20
   |
13 |     let s1 = String::from("Hello world");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
...
17 |         let s2 = s1;
   |                  -- value moved here
...
21 |     println!("{}", s1);
   |                    ^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
17 |         let s2 = s1.clone();
   |                    ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

为了理解这个编译错误，我们需要理解程序在内存上都干了什么：

对于一个**String**类的变量，程序会在内存中储存三个信息：**ptr, length and capacity**，这三个信息本身储存在栈内存中。而指针所指向的具体的内存（就是实际的data）是分配在堆内存中的，在进行赋值操作的时候，程序会将指针，长度和capacity这三个变量的值都拷贝一份给新的变量s2,但是**不会开辟一块新的内存**给s2使用！（这就是严重的**对指针变量的浅拷贝**）

这会带来很严重的问题，例如当两个变量各自离开自己作用域的时候，变量的所有内存会被销毁返回给操作系统，而这就意味着所开辟的那一块堆内存会被**释放两次**！这就是非常著名的**Double Free Bug**（二次释放）

对于C++来说，解决的办法是**手动设计String类的拷贝构造函数**，使其复制一块新的内存给s2，这样实现深拷贝就可以让两个变量互不干扰，独立活动。（现代C++也设计了移动语义来应对**不需要复制**的高效率场景）而对于Rust来说，其解决方案是让赋值后的s1失效。也就是说，当s1离开作用域的时候，原先的字符串数据已经不再属于他了，程序也不需要再释放新的内存。

> 这就是**移动语义**，到底是谁抄谁的呢。。。

### Clone

可否进行补救？当然可以，Rust提供了`clone()`函数来实现深拷贝。

```rust
/*
 * @Author: Xiyuan Yang   xiyuan_yang@outlook.com
 * @Date: 2025-04-16 19:57:16
 * @LastEditors: Xiyuan Yang   xiyuan_yang@outlook.com
 * @LastEditTime: 2025-04-19 01:19:36
 * @FilePath: /Rust/ownership/src/main.rs
 * @Description:
 * Do you code and make progress today?
 * Copyright (c) 2025 by Xiyuan Yang, All Rights Reserved.
 */

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

输出结果：

```
s1: Hello world
Entering the scope of s2
s2: Hello world
Hello world
```

{% note primary %}

**对于一些非常简单的值，深拷贝和浅拷贝是等价的，因为数据本身就储存在栈上面**，不涉及堆内存的管理。

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

### Trait

- Copy Trait 可以使用于**像整数完全可以被存放在stack上面的类型**。
- 如果一个类实现了Copy Trait，那么旧的变量在赋值之后仍然可以被使用。
- 如果一个类实现了Drop Trait，那么Copy Trait将不被允许实现。
- 如果一个元组的所有字段都是可以被copy的，那么其本身也是可以被copy的。

### 所有权与函数

本质上和变量赋值完全类似，**可以进行移动或者复制**。

- 对于Copy Trait，函数的传参就是复制的过程，不会影响原变量的内容。
- 但是对于非 Copy Trait，就相当于C++中的&&右值引用，直接交出了变量的所有权。

函数的返回值也是如此。

## 引用和借用

Rust所有权机制设计的初衷是希望**避免出现内存泄漏等C++中老大难的问题**，但是这样的设计会带来非常大的麻烦，尤其是在面对函数的反复传参的过程中。我们来看下面的代码：

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

同样，我们也可以修改**函数的参数和返回值列表**，让对应的临时变量返回所有权。

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

实际上，**Rust**还实现了第三种方式，即在函数内部（临时作用域）中进行变量的借用。（有点类似于C++中的reference）

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

- **不可以同时拥有一个可变引用和一个不可变引用**。（允许只存在多个不可变的引用）

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

