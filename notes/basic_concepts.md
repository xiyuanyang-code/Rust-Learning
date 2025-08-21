# Basic Concepts in Rust

## 变量和常量

### 变量：默认不可变性

```bash
rustc --explain E0384
```

```text
An immutable variable was reassigned.

Erroneous code example:

fn main() {
    let x = 3;
    x = 5; // error, reassignment of immutable variable
}

By default, variables in Rust are immutable. To fix this error, add the keyword mut after the keyword let when declaring the variable. For example:

fn main() {
    let mut x = 3;
    x = 5;
}

Alternatively, you might consider initializing a new variable: either with a new bound name or (by shadowing) with the bound name of your existing variable. For example:

fn main() {
    let x = 3;
    let x = 5;
}
```

在 Rust 中对**不可变性**具有严格的要求，因此解决办法为：

- 在变量声明之处使用 `mut` 关键词，在这之后就允许变量值的更改。

- 使用新变量覆盖（shadowing）现有变量，覆盖的变量可以是可变的也可以是不可变的。

    - 和 C++ 一样，可以使用大括号限定遮蔽的作用域，在当前作用域下，对应变量的值就会被遮蔽。
    - 遮蔽相比于 `mut` 来说更强大：灵活的作用域 & 可以变换类型（本质上遮蔽其实就是创建了一个新的同名变量）

### 常量

常量的声明比不可变变量要更加严格，核心原因在于**常量在编译时绑定而不是在运行时绑定**：

- 在编译时绑定而非运行时绑定

    - 这就导致了常量的值不可以**被覆盖**，即 shadowing 在常量上是不被允许的。

    - 并且，这个量不可以是运行时计算的结果（例如一个函数的返回值）

- 常量始终存在于全局作用域中，并且在整个程序的生命周期中都有效。

- **必须要类型注解**。

## 数据类型

Rust 是一种静态类型的语言，即**在程序编译阶段**所有变量的类型都必须被确定并且检查通过。但是和 C++ 不同的是，Rust 引入了 `let` 关键字，这样创建新变量的过程中不需要显示地规定变量类型。因此，在做类型转换的时候，需要**显示定义类型注解**来防止编译器陷入困惑。（尤其在类型转换的结果有很多可能的时候）

Rust 的数据类型可以分为两大类：**标量类型 (Scalar Types)** 和 **复合类型 (Compound Types)**。

### 标量类型 (Scalar Types)

标量类型代表一个单一的值。Rust 内置了四种主要的标量类型：整数、浮点数、布尔值和字符。

#### 整数 (Integers)

整数类型用于存储没有小数部分的数字。Rust 提供了多种整数类型，每种都具有指定的大小（位数）和符号（有符号或无符号）。

  * **有符号整数 (`i`)**: 可以是正数、负数或零。类型包括 `i8`, `i16`, `i32`, `i64`, `i128`。
  * **无符号整数 (`u`)**: 只能是正数或零。类型包括 `u8`, `u16`, `u32`, `u64`, `u128`。
  * **架构相关类型 (`isize`, `usize`)**: 它们的位数取决于你的电脑架构。在 64 位系统上，它们是 64 位；在 32 位系统上，它们是 32 位。`usize` 主要用于数组索引和大小，因为它能保证在任何系统上都能表示内存中的所有位置。

#### 浮点数 (Floating-Point Numbers)

浮点数用于存储有小数部分的数字。Rust 有两种浮点类型：

  * `f32`: 32 位单精度浮点数。
  * `f64`: 64 位双精度浮点数，是默认的浮点类型。

#### 布尔值 (Booleans)

布尔类型只有两个可能的值：`true` 或 `false`。它通常用于条件判断。

#### 字符 (Characters)

Rust 的 `char` 类型是一个 Unicode 字符，而不是简单的 ASCII 字符。这意味着它可以表示任何语言的字母、数字、标点符号，甚至是表情符号。`char` 类型占 4 个字节。

```rust
let my_char = '❤️';
```


### 复合类型 (Compound Types)

复合类型可以将多个值组合成一个类型。Rust 提供了两种基本的复合类型：元组和数组。

#### 元组 (Tuples)

元组是一种将多个不同类型的值打包成一个复合类型的方法。一旦创建，它的长度就不能改变。

  * 你可以使用索引来访问元组的元素。
  * 元组的类型由其内部元素的类型和顺序决定。

<!-- end list -->

```rust
let my_tuple = (500, 6.4, "hello");
let (x, y, z) = my_tuple; // 解构
let first_item = my_tuple.0; // 使用索引访问
```

#### 数组 (Arrays)

数组是一种将多个**相同类型**的值放入一个固定长度的集合中的方式。

  * **数组的长度是固定的，不能改变**。
  * 数组的类型由**元素的类型和长度**共同决定，例如 `[i32; 5]` 代表一个包含 5 个 32 位整数的数组。
  * 数组的元素在内存中是连续存储的。
  * 数组的内存在栈上分配。

<!-- end list -->

```rust
let my_array: [i32; 5] = [1, 2, 3, 4, 5];
let first_element = my_array[0]; // 使用索引访问
```

除了这些基本的数据类型，Rust 还提供了更高级的类型，比如 **切片 (`&[T]`)**、**字符串 (`String` 和 `&str`)**、**枚举 (`enum`)** 和 **结构体 (`struct`)** 等。这些类型构成了 Rust 强大而安全的类型系统的基础。

> 一个复合类型的大小是其所有字段大小的总和，加上为了满足对齐要求而产生的填充（padding）。这一点和 C++ 保持一致。

```text
--- 标量类型 ---
i8:    1 bytes
u8:    1 bytes
i16:   2 bytes
u16:   2 bytes
i32:   4 bytes
u32:   4 bytes
i64:   8 bytes
u64:   8 bytes
i128:  16 bytes
u128:  16 bytes
isize: 8 bytes
usize: 8 bytes
f32:   4 bytes
f64:   8 bytes
bool:  1 bytes
char:  4 bytes

--- 复合类型 ---
():      0 bytes
(i32, f64): 16 bytes
[i32; 3]: 12 bytes
```

### 溢出 

在 Rust 中，溢出现象仍然存在。

- 在 Debug 模式下会导致程序 panic
- 在 release 模式下程序不会报错

这就是 Rust 比 C++ 更安全的体现之一！与此同时，Rust 还设计了一些函数，来更安全的**显示处理溢出的可能性**。

在 Rust 中，`wrapping_add`、`checked_add`、`saturating_add` 和 `overflowing_add` 是整数类型上的一组方法，它们提供了对**整数溢出**行为的显式控制。

这些方法让你能够清晰地表达自己的意图，而不是依赖于 Rust 在不同编译模式下的默认行为。

#### `wrapping_add`（环绕加法）

**功能**：当计算结果超出整数类型的范围时，它会**环绕**（或称“回卷”），从最小值或最大值重新开始。

**用途**：

  * 处理需要模块化算术（Modular Arithmetic）的场景，比如哈希函数、加密算法、循环数组索引等。
  * 在发布模式下，这是 Rust 整数溢出的默认行为。

**示例**：

```rust
let x: u8 = 250;
let y = 10;
let result = x.wrapping_add(y); // 250 + 10 = 260
                                // 260 超过了 u8 的最大值 255
                                // 结果会环绕：260 - 256 = 4
println!("Result: {}", result); // 输出：4
```

#### `checked_add`（检查加法）

**功能**：执行加法，并在发生溢出时返回一个 `Option` 类型。如果结果没有溢出，返回 `Some(result)`；如果溢出，返回 `None`。

**用途**：

  * 这是最安全的处理溢出的方法，让你有机会在运行时优雅地处理错误，而不是让程序直接恐慌。
  * 非常适合需要确保计算结果不会溢出的场景。

**示例**：

```rust
let x: u8 = 250;
let y = 10;
let result = x.checked_add(y);

if let Some(sum) = result {
    println!("Sum is {}", sum);
} else {
    println!("Overflow occurred!"); // 发生了溢出，打印此行
}
```


#### `saturating_add`（饱和加法）

**功能**：当计算结果超出整数类型的范围时，它会将结果\*\*“饱和”\*\*到该类型的最大值或最小值。

**用途**：

  * 防止意外的环绕行为，将结果限制在可接受的范围内。
  * 常用于图形处理、音频处理或任何需要防止数据“回卷”的场景。

**示例**：

```rust
let x: u8 = 250;
let y = 10;
let result = x.saturating_add(y); // 250 + 10 = 260
                                  // 260 超过了 u8 的最大值 255
                                  // 结果会被“饱和”到 u8 的最大值 255
println!("Result: {}", result); // 输出：255
```


#### `overflowing_add`（溢出加法）

**功能**：执行加法，并返回一个包含**计算结果**和**是否溢出**的元组。

**用途**：

  * 需要同时获取计算结果和溢出状态的场景。
  * 通常用于自定义溢出逻辑或调试。

**示例**：

```rust
let x: u8 = 250;
let y = 10;
let (result, overflowed) = x.overflowing_add(y);

println!("Result: {}, Overflowed: {}", result, overflowed);
// 250 + 10 会环绕到 4，并标记为溢出
// 输出：Result: 4, Overflowed: true
```

> Panic in Rust
> 在 Rust 中，Panic 是一种不可恢复的错误状态，它表明程序遇到了一个非常严重的问题，无法继续安全地执行下去。当 Rust 程序发生 panic 时，它会：

- 打印一条错误信息。
- 清理栈（unwind the stack），释放所有函数栈帧所拥有的资源。
- 最终，程序会直接终止。
- 可以把 panic 理解为程序对自己说：“我不知道该如何处理这个致命的错误，继续运行下去只会让情况更糟，所以我选择干净利落地退出。”

## 函数

### 函数的参数

- 同样分为形式参数和实际参数
- 和 C++ 一样，需要显示提供类型注解

### 语句和表达式

> 这是一个很有意思的问题，因为 C++ 和 Rust 在这个地方存在很大的不同。

#### Definition

- 表达式：任何**能够求值并产生一个值**的代码:
    - `x + y`
- 语句：执行指令但是不产生任何的值，在 C++ 中，语句通常以**分号**结尾。
    - 一般表达式作为语句的一部分。

Rust 是一门**基于表达式**的语言，在 Rust 中，几乎所有东西都是表达式，这意味着它们都会产生一个值。

- Rust 的 if 块、match 块和 {} 块都可以返回值。

- 函数的最后一行（**不带分号**）默认是返回值。

例如下面的一段代码：

```rust
fn main() {
    println!("Hello world!");
    expression();
}

fn expression() {
    let x = if 1 == 2 { 5 } else { 10 };
    let y = {
        let z = 1;
        z + 2
    };
    println!("x value is {}", x);
    println!("y value is {}", y);
}
```

- 使用 let 实现变量的创建过程是一个语句而不是表达式（因为这个过程本身不会产生返回值，并且最后使用分号结尾）
- 但是使用 `{}` 括起来的部分是一个表达式（同时，他还是限定范围的作用域的开始），他的返回值是该代码块中最后一个表达式的返回值。
    - 因此如果在上面的代码中多加了一个分号，写成了 `z + 2;`，那这个代码块的返回值就是**空**，是一个空元组，会导致编译失败。
    - 这也可以作为函数返回值的设计，同样，if else 块也遵循相同的返回值返回原理。
    ```rust
    fn pow(x: i32, p: u32) -> i64 {
    let mut result: i64 = 1;
    for _i in 0..p {
        result = result * (x as i64);
    }
    result
    }
    ```

## 控制流

- if & else & else if
    - 和 C++ 不同的是，if 后的 expression 判断**必须是一个布尔值**，他不会像C++一样执行**布尔值和整数的类型自动转换**（非0都是 True）

    ```rust
    fn if_else() {
    let x = 1;
    if x {
        println!("It is True");
    }
    }
    ```

- loop：无限循环，但是可以添加 break 表达式。
    - 循环标签：可以将 break 和 continue 应用于指定层数的循环而不是最外层的循环。
    ```rust
    fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Counting up end.")
    }
    ```

- while 循环：同样可以使用循环标签
- for 循环：**遍历数组**，集合等。
