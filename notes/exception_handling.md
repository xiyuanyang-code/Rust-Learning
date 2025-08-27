# Exception Handling in Rust
> Rust 中的错误处理机制

## 错误分类 & 异常处理的哲学

- 可恢复的错误通常是那些可以由程序本身处理并继续运行的错误。这些错误往往是由环境或用户输入等外部因素引起的，而不是编程逻辑上的缺陷。Rust 使用 `Result<T, E>` 枚举来处理这类错误。

- 不可恢复的错误是指那些表明程序存在严重缺陷、无法继续安全运行的错误。这些错误通常是由于编程逻辑中的 Bug 导致的，例如访问数组越界、除以零等。Rust 使用 `panic!` 宏来处理这类错误，它会直接导致程序崩溃并打印出错误信息和调用栈。

> 和 C++ 一样，Rust 同样也有一套完整的错误处理的机制。

### 异常处理的哲学

异常处理顾名思义，程序需要足够鲁棒来应对除了 Happy Path 之外的异常情况，面对上面两种错误分类，异常处理的哲学存在显著的区别：

{% note primary %}

**Fail Fast & Fail Gracefully**

- 可恢复错误是外部环境或用户行为导致的，而**不是程序自身的逻辑缺陷**。处理这类错误的核心是**优雅地失败**（Fail Gracefully），让程序能够捕获错误，然后采取补救措施，比如提示用户、重试操作或使用默认值，而不是直接崩溃。

- 不可恢复的错误(数组越界、内存访问错误、调用了处于无效状态的函数)是程序员的 Bug，而不是外部环境问题。处理这类错误的核心是**快速失败**（Fail Fast），让程序立即崩溃，并提供足够的信息（如堆栈跟踪），以帮助开发者迅速定位并修复 Bug。

{% endnote %}

## `panic!`

`panic!` 是一个宏（和 `println!` 一样）。在代码中调用 `panic!` 会导致程序进入 panic 状态（效果和代码中出现严重错误时一样）。在这样的情况下，程序会打印错误信息，**展开**并且清理栈数据，然后退出。

> panic 只能用来处理不可恢复的错误，一般这些错误说明你的代码逻辑出现严重的 Bug，导致程序无法正常运行，此时最应该做的就是**输出相关提示信息然后迅速退出，修复 Bug**。
> panic 是 Rust 安全性的最后一道护城河，相比于 C++ 的 UB 来说，安全性会高很多。

```toml
[profile.release]
panic = "abort"
# abort directly without unwinding
```

```rust
fn main() {
    println!("Hello world!");
    panic!("Oh No! It will crash");
}
```

```text
Hello world!

thread 'main' panicked at src/bin/exceptions.rs:3:5:
Oh No! It will crash
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:697:5
   1: core::panicking::panic_fmt
             at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/core/src/panicking.rs:75:14
   2: exceptions::main
             at ./src/bin/exceptions.rs:3:5
   3: core::ops::function::FnOnce::call_once
             at /home/xiyuanyang/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

除了显示调用 `panic!` 之外，代码本身出现严重的漏洞也会引发程序的 panic。

```rust
fn main() {
    println!("Hello world!");
    // panic!("Oh No! It will crash");

    let v = vec![1, 23, 4, 5, 5];
    // this will cause error, but not a compile error
    // index out of bounds: the len is 5 but the index is 100
    println!("{}", v[100]);
}
```

这是一个经典的数组越界的问题，在 C++ 中会导致 Segmentation Fault（非法的内存访问），在 Rust 中会直接报错 index out of bounds。（当然因为我们使用了封装好的容器）

## Result

对于可恢复的错误，Rust 同样引入了一套相当优雅的异常处理机制。

```rust
// definition of Result enums
pub enum Result<T, E> {
    /// Contains the success value
    #[lang = "Ok"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    #[lang = "Err"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}
```

以文件读写为例子：

```rust
fn test_file_open(){
    let result = File::open("./expressions.rs");
    println!("{result:?}");
}
```

```text
Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
```

```rust
fn test_file_open(){
    let result = File::open("./README.md");
    println!("{result:?}");
}
```

```text
Ok(File { fd: 3, path: "/home/xiyuanyang/ProgrammingLang/Rust/rust-learning/README.md", read: true, write: false })
```

我们来看函数签名：

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new().read(true).open(path.as_ref())
}
```

他返回的最终是一个枚举类型（泛型），并且如果文件读写成功，返回的是包含对应文件的句柄的 OK 实例。更进一步的，我们可以使用 match 块来对应的处理不同的结果类型。

```rust
fn test_test_file_open(){
    let file_list = vec!["./README.md", "./readme.md"];
    for file in &file_list{
        println!("Trying to read {}...", file);
        let result = File::open(&file);
        let greeting_file = match result {
            Ok(file) => file,
            Err(error) => panic!("Error! Some error occur {error:?}")
        };
    }
}
```

上文的代码利用 match 块做了细致的分类，将 Happy Path（文件读取成功）和 Sad Path（文件读取失败）对应的路径做了清晰的切割。在 Error 内部，我们还可以进一步细分，让程序员在捕获异常时得到更多的调试信息。

```rust
fn test_test_file_open() {
    let file_list = vec!["./README.md", "./readme.md"];
    for file_name in &file_list {
        println!("Trying to read {}...", file_name);
        let result = File::open(&file_name);
        let greeting_file = match result {
            Ok(file) => {
                println!("Loading file successfully!");
                file
            }
            Err(error) => {
                println!("Error! Trying to create a new file");
                match error.kind() {
                    ErrorKind::NotFound => match File::create(&file_name) {
                        Ok(fc) => fc,
                        Err(error) => panic!("Error creating file: {error:?}"),
                    },
                    _ => {
                        panic!("Other problems")
                    }
                }
            }
        };
    }
}
```

上文的代码的缩进和逻辑链条非常的清晰，但是唯一的缺点就是代码嵌套太深，影响了可读性。Rust 高级编程引入了 闭包 的概念（也是函数式编程的核心之一），可以完美替代上文的 match。同时，标准库也对异常处理做了一些必要的封装，让你不需要每次都手动 match 具体的返回结果。

### `unwrap`

Rust 中的 unwrap 函数是 Option 和 Result 枚举类型的一个方法，它用于从这些枚举中提取值。它的核心作用是方便地获取一个可能存在或可能不存在的值。（相当于一层封装，可以少写一个 match）

- 如果 Result 是 `Ok(T)`，它会返回其中的值 T。

- 如果 Result 是 `Err(E)`，它会立即导致程序 panic。

```rust
fn unwrap_test() {
    let file = File::open("./README.md").unwrap();
}
```

来看一看源代码：

```rust
#[inline(always)]
#[track_caller]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn unwrap(self) -> T
where
    E: fmt::Debug,
{
    match self {
        Ok(t) => t,
        Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
    }
}
```

### `expect`

和 `unwrap` 的功能完全一致，它让你的**程序崩溃时能提供更有用的上下文信息**。这对于调试和快速定位问题非常有帮助。

> 这个非常有用，可以输出自定义的提示信息。

```rust
fn expect_test(){
    let file = File::open("./rEADME.md").expect("Error, this is a test panic message");
}
```

