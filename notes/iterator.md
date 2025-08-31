# Functional Programming in Rust: Closures and Iterators

{% note primary %}

Closure 和 Iterators 本身并不是 Rust 专属的概念，或者说是因为 Rust 本身支持甚至倾向函数式编程的设计进而支持这两种函数式编程的高级技巧。因为笔者本人在学习 Advanced Pythonic 的时候本身就学过了这些函数式编程的概念，因此本文并不将重点放在抽象而脱离语言的概念解释上，而是将重心转移到在 Rust 独有的语言特性中的具体使用。

{% endnote %}

## Closure

在函数式编程中，**闭包**（Closure）是一个非常核心且强大的概念。简单来说，一个闭包就是一个函数以及它被创建时所处的词法环境的组合。

这里的“词法环境”指的是在函数定义时，它能够访问的所有局部变量、参数以及外部函数中的变量。当这个内部函数从外部函数中被返回时，即使外部函数已经执行完毕，它的词法环境并不会被销毁，而是会和内部函数一起被“打包”起来，形成一个闭包。这使得闭包能够“记住”和访问它在创建时所处的环境，即使该环境已经不存在于调用栈中。

> 在 Python 中，闭包的重要应用在于**外部函数中的内部函数**，具体的使用应用场景包括**装饰器**、函数工厂等。

### 使用闭包捕获环境

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

闭包的设计关键在于 `giveaway` 函数的设计上。

```rust
fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
}
```

在 Rust 中，**闭包**是一种匿名函数，它可以捕获其所在环境中的变量。在这里，闭包就是 `|| self.most_stocked()`。

- 作为一个匿名函数出现。
- **捕获环境中的变量**：闭包 `|| self.most_stocked()` 捕获了其外部环境中的 `self` 变量。这意味着闭包可以在被调用时，访问并使用 `Inventory` 实例中的 `shirts` 数据。
让我们把它拆解开来解释：

`unwrap_or_else` 是一个 `Option` 类型的方法，它的工作机制如下：

1.  如果 `user_preference` 是 `Some(value)`，那么 `unwrap_or_else` 会忽略闭包，直接返回 `value`。
2.  如果 `user_preference` 是 `None`，那么 `unwrap_or_else` 就会**调用这个闭包**。当 `unwrap_or_else` 调用闭包 `|| self.most_stocked()` 时，闭包会执行 `self.most_stocked()` 方法，并返回计算结果。这个结果（`ShirtColor`）将作为 `giveaway` 函数的返回值。

使用匿名函数对于开发者而言具有独特的优势，比如不用写复杂的参数列表的类型注解。因为闭包和上下文是高度相关的（捕获外部变量），因此编译器在绝大多数情况下可以推断出编译器的类型。

### 闭包匿名函数的定义

```rust
fn main() {
    println!("Hello world!");
    let add_one = |x: i32| x + 1;
    let result = add_one(1);
    println!("{}", result);
}
```

### 捕获引用 & 移动所有权

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```

Rust 的闭包的强大之处不仅仅在于匿名函数的设计，更在于**外部作用域中变量的捕获**。例如 `only_borrows` 的闭包使用了外部作用域中的 Vec 数组。在这里，实际上**闭包捕获了其不可变的引用**，这也是闭包的默认行为。

