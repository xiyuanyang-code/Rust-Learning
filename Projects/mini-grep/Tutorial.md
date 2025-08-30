# MiniGrep Tutorial for Rust

## 接受命令行参数

这一部分需要读取参数值。作为整个程序的入口，可以直接写在 `main.rs` 的主函数里。

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
```

## 