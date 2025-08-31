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


同样，可以使用 `mut` 关键词声明可变的闭包，会捕获一个对外部变量的**可变引用**，进而允许修改对应外部变量的值。

```rust
fn main() {
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");
        let only_borrows = || println!("From closure: {list:?}");
        println!("Before calling closure: {list:?}");
        only_borrows();
        println!("After calling closure: {list:?}");
    }

    {
        let mut mut_list = vec![1, 2, 3];
        println!("Before defining closure: {mut_list:?}");
        let mut mut_borrows = || {
            mut_list.push(23);
            println!("After modification in the closure: {mut_list:?}");
        };
        // it will create a mutable reference
        // println!("Before calling closure: {mut_list:?}");
        // not allowed here
        mut_borrows();
        println!("After calling closure: {mut_list:?}");
    }
}
```

以上是闭包**捕获外部变量的引用的例子**，无论是可变的引用还是不可变的引用。但是有时候我们希望直接获取外部变量的所有权，例如将闭包传递给一个新的线程。（引用会带来复杂的生命周期问题，这在多线程管理的问题上更加的复杂）

```rust
let list = vec![1, 3, 4];
println!("Before defining: {list:?}");
thread::spawn(move || println!("From thread: {list:?}"))
    .join()
    .unwrap()
```

- `thread::spawn(...)`：这是 Rust 标准库中用于创建一个**新线程**的函数。它接收一个**闭包**作为参数，并在这个新线程中执行该闭包。
- `move`：这个关键字至关重要。它强制闭包获取它所使用的**所有外部变量的所有权**。在这个例子中，闭包内部没有使用任何外部变量

### Fn `Traits`

上文介绍了三种闭包的基本使用方式：

- 不可变借用（默认）
- 可变借用（在闭包创建时创建一个可变借用）
- 移动（移动所有权）

在原理层面，闭包之所以能够捕获其环境中的变量，是因为它们实现了特殊的 **`Fn` traits**。

* `FnOnce`（移动所有权）：只能调用一次。
* `FnMut`（可变借用）：可以多次调用，可以修改变量。
* `Fn`（不可变借用）：可以多次调用，只可读取变量。


这些 traits 定义了闭包可以以何种方式被调用，并决定了它们如何与捕获的变量进行交互（借用还是移动）。Rust 中有三种主要的 `Fn` traits：`Fn`、`FnMut` 和 `FnOnce`。它们形成一个层次结构，其中 `Fn` 是最通用的，`FnOnce` 是最不通用的。

  * `FnOnce` `trait` 实现了 `FnMut`。
  * `FnMut` `trait` 实现了 `Fn`。

这意味着任何可以作为 `Fn` 使用的闭包，也可以作为 `FnMut` 或 `FnOnce` 使用。

#### `FnOnce`

* **捕获方式**：`FnOnce` 闭包会**获取被捕获变量的所有权**。
* **调用次数**：它只能被**调用一次**。一旦调用，它就会消耗掉自身和它捕获的变量。

```rust
let s = String::from("Hello");

let consume_s = move || {
    println!("{}", s);
    // 在这里，s 的所有权被移动到闭包中
};

// 只能调用一次
consume_s();

// 下面这行会报错，因为 consume_s 已经被调用并消耗了
// consume_s();
```

#### `FnMut`

* **捕获方式**：`FnMut` 闭包以**可变借用**的方式捕获变量。
* **调用次数**：可以被**多次调用**，并且每次调用都可以修改其捕获的变量。

```rust
let mut counter = 0;

let mut increment_counter = || {
    counter += 1;
};

// 可以多次调用
increment_counter();
increment_counter();
println!("Counter: {}", counter); // 输出: Counter: 2
```

#### `Fn`

* **捕获方式**：`Fn` 闭包以**不可变借用**的方式捕获变量。
* **调用次数**：可以被**多次调用**，但**不能修改**其捕获的变量。

```rust
let num = 5;

let check_num = || {
    println!("Number is: {}", num);
};

// 可以多次调用
check_num();
check_num();
```

例如下面的例子，对一个列表进行原地的排序操作：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{}", num_sort_operations);
    // num: 6
    
    println!("{list:#?}");
}
```

`sort_by_key()` 的函数需要接受一个闭包，这个闭包的函数返回值作为列表排序的依据。这个闭包允许捕获变量的修改并且需要多次调用，因此实现的是一个 `FnMut` Trait.


## Iterator

迭代器在惰性求值等领域中具有极高的性能优势。

```rust
// get an iterator
fn main() {
    println!("Hello world!");
    // generate an iterator
    let v1 = vec![1,2,3,4];
    let v1_iter = v1.iter();
    for val in v1_iter{
        println!("Get: {}", val);
    }
}
```

和 Python 一样，迭代器的关键在于 `next()` 函数。具体到 Rust 而言，需要实现一个标准库定义的 Iterator 的 Trait。

除了最基本的调用 next 的方法，Rust 还支持一系列**消费适配器**的实现，这些适配器会得到迭代器的所有权并且消耗这个迭代器。

```rust
// it is functional programming!
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
println!("{v2:?}");
```

