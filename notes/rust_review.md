# Rust Review

> 重新开始 Rust 的学习之旅...

## Hello World

```rust
// hello world to all of you guys!

fn main(){
    println!("Hello world!")
}
```

`println!` 是 Rust 中的**宏**（严格意义上来说这并不是函数）。在 Rust 中，宏（Macros）是一种元编程工具，它允许你编写能够生成代码的代码。简单来说，宏不是在运行时执行的函数，而是在编译时展开的，它们会把你的宏调用替换成实际的 Rust 代码。

- 宏的主要作用是减少重复代码。如果你发现自己在写大量相似或重复的模式化代码，就可以考虑用宏来自动化这个过程。

- **编译时展开**：这是宏和函数最大的区别。函数在程序运行时被调用，而宏在编译时被处理。编译器看到一个宏调用，就会根据宏的定义来生成一段新的代码，然后用这段新代码替换掉原来的宏调用。（有点类似于 C++ 中的 `define`，但是 `define` 只是简单的文本替换，Rust 的宏比这个要更高级并且更加安全）

- 宏调用通常以感叹号 ! 结尾，比如我们常见的 println! 和 vec!。这个感叹号是 Rust 宏的特有标记，用于区分宏和普通的函数。

宏在 Rust 中非常重要，因为它解决了许多函数无法解决的问题：

- 可变参数：函数在 Rust 中**不能接受可变数量的参数**，但宏可以。例如，println! 可以接受任意数量的参数。

- 生成结构体或枚举：宏可以根据你的输入，生成整个结构体（struct）或枚举（enum）的定义。这在实现像序列化（Serde）这样的复杂功能时非常有用。


> TOML (Tom's Obvious, Minimal Language), 这个名字好有趣。

## Cargo

- `cargo build`：编译

- `cargo run`：编译并运行

- `cargo check`

- `cargo clean`


## Guessing Number Game

- mutable variables and immutable variables.
    - 变量绑定

    ```rust
    let mut guess = String::new();
    ```

    - guess 实现了对 String 类的实例化，并且被绑定了一个值，这个值有 String 类的 new 函数提供。（**关联函数**）

> 在 Rust 中，关联函数（associated functions）是与**特定类型（比如结构体 struct 或枚举 enum）相关联的函数**，但它们不需要 self 参数。换句话说，这些函数是直接通过类型本身来调用的，而不是通过该类型的一个实例（instance）。关联函数直接通过类型的 `::` 来调用。有点类似于 C++ 中的**静态成员函数**。

> 关联函数最常见的例子就是用于创建新实例的构造函数。

- 输入文件句柄：`std::io::stdin()`返回一个实例。

- **引用**：Rust 的核心机制！

- `expect`: read_line 会将⽤⼾输⼊附加到传递给它的字符串中，不过它也会返回⼀个类型为 **Result** 的值。 Result 是⼀种枚举类型，通常也写作 enum，它可以是多种可能状态中的⼀个。我们把每种可能的状态称为⼀种 **枚举成员**（variant）

    - Ok，Err 是其两个枚举成员。

    - Result 类型的值，像其他类型⼀样，拥有定义于其实例上的⽅法。 Result 的实例拥有 expect ⽅法。如果 io::Result 实例的值是 Err ， expect 会导致程序崩溃，并**输出当做参数传递给 expect 的信息**。所以当 read_line ⽅法返回 Err ，则可能是来源于底层操作系统错误的结果。如果 Result 实例的值是 Ok ， expect 会获取 Ok 中的值并原样返回。（返回值不等于输出信息）

    - 一种经典的**错误处理机制**。

- crate
    - 在 Rust 中，crate 是一个编译单元，是 Rust 代码的最小打包和编译单位。每一个 Rust 项目在编译时都会被视为一个或多个 crate。

    - Binary crate：对应一个可执行文件，需要有 main 函数作为函数入口。

    - Library crate：对应库文件，不包含 main 函数，但是可以被其他 crate 引用。

    - [Crates.io](https://Crates.io)：Rust 开源发布平台

- trait

    - 在 Rust 中，Trait 是一个核心概念，它定义了类型可以共享的功能。你可以把它理解为一组方法的签名集合，这些方法可以被不同的类型实现。简单来说，Trait 就像是编程世界的“接口”或“协议”。它规定了“**如果一个类型拥有这个 Trait，那么它必须提供这些方法**”。（有点类似于抽象基类，但是没有类的继承）、

    - trait 的设计来源于 Rust 本身的作用域规则：在 Rust 中，即使一个类型（比如 rand::rng() 返回的那个类型）实现了某个 trait，**你也不能直接调用该 trait 定义的方法，除非你显式地将这个 trait 引入到你的作用域中**。这是 Rust 的作用域规则。

        - 这是 Rust 的孤儿规则（Orphan Rule）和一致性（Coherence）设计的一部分，其核心目的是为了**防止类型和 trait 的实现冲突**。
- 类型转换：`let guess: u32 = guess.trim().parse().expect("Please type a number.");`。
    - **Shadowing**：变量内容的遮蔽
    
