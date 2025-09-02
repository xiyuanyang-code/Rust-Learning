# Smart Pointer in Rust

Rust 的设计理念是**内存安全**，因此并不提倡在代码中直接使用**裸指针**来直接操作对象，而是提倡使用更加安全的**智能指针**。

## `Box <T>` 指向堆上的数据

- 对于基本的数据类型，数据会直接储存在栈内存上。
- 使用智能指针允许你指向一个堆内存上的数据，而将指针的值储存在栈内存上。
    - 实际上，String 类和 Vec 本质上也是**智能指针的高级封装**！

```rust
use std::any::type_name;
fn main() {
    println!("Hello world!");
    intro_box();
}

fn print_type<T>(_:  &T) -> &'static str {
    type_name::<T>()
}


fn intro_box(){
    let b = Box::new(5);
    println!("The type of b is {}", print_type(&b));
    println!("b = {b}");
}
```

```text
Hello world!
The type of b is alloc::boxed::Box<i32>
b = 5
```

值得注意的一点是，虽然 i32 类型是可以直接存储在栈上的，但是因为智能指针的设计，这个变量被存储在了堆上。使用指针可以实现链表和**带有嵌套的结构体类型**。就像 C++ 中链表实现的方式一样。在 Rust 中，这样的数据结构叫做 cons list（来源于 Lisp）,这样可以解决这些嵌套类型的大小在编译时无法确定的问题。

```rust
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn use_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
}
```

## Deref Trait

在我们处理常规的引用时，我们可以使用**解引用运算符**实现基本的对引用的解引用。（引用的本质就是一个指针，这里 Rust 的行为和 C++ 的行为存在显著不同）。实现 Deref trait 允许我们定制解引⽤运算符。

同样的，智能指针也可以使用解引用运算符。

```rust
fn deref_trait(){
    let x = 50;
    let ref_1 = &x;
    let y = Box::new(x);
    println!("{}", x == *ref_1);
    println!("{}", x == *y);
    println!("{}", *y == *ref_1);
    // println!("{}", y == ref_1);
    // error: different types
}
```

如果我们需要自定义一个指针类型并且需要自定义操作符，则需要手动实现该类型的 Deref Trait。

```rust
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

例如，String 类实现的 Deref Trait 是这样的：

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Deref for String {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}
```

这就是为什么 String 类可以处理可变的类型！因为可变长度的 str 存储在堆数据上（实际上是一个 Vec），最终解引用返回的是一个 &str。

```rust
fn deref_string_2(){
    let s = String::from("Hello world");
    // Dereference s to get a &str, then get the type name of that reference.
    let x = &*s;
    println!("{}", print_type(&x));
}
```

{% note primary %}

`&str` (字符串切片) 本身是存储在**栈上**的。`&str` 是一种**胖指针（fat pointer）**，它由两部分组成：

1.  一个指向字符串数据的**指针**。这个指针存储了字符串数据在内存中的起始地址。
2.  一个表示字符串长度的**usize**值。

`&str` 指向的字符串数据本身可以存储在不同的位置：

* **静态内存（Static Memory）**：最常见的情况。当你使用字符串字面量（`let s = "Hello";`）时，这个字符串数据被硬编码到你的程序二进制文件中，并加载到程序的静态内存区域。`&'static str` 中的 `'static` 生命期就表示它在整个程序运行时都有效。
* **栈上（Stack）**：很少见，例如从栈上的字节数组创建 &str。
* **堆上（Heap）**：当你从一个堆上的 `String` 中切片时，`&str` 会指向堆上的数据。例如：`let s = String::from("Rust"); let slice = &s[0..2];`。这里的 `s` 存储在堆上，`slice` 指向堆上 `s` 的一部分。

{% endnote %}

### Deref 强制转换

Deref 强制转换（deref coercions）将实现了 Deref trait 的类型的引⽤转换为另⼀种类型的引⽤。例如，Deref 强制转换可以将 `&String` 转换为 `&str` ，因为 String 实现了 Deref trait 因此可以返回 &str 。Deref 强制转换是 Rust 在函数或⽅法传参上的⼀种便利操作，并且只能作⽤于实现了 Deref trait 的类型。当这种特定类型的引⽤作为实参传递给和形参类型不同的函数或⽅法时将⾃动进⾏。这时会有⼀系列的 deref ⽅法被调⽤，把我们提供的类型转换成了参数所需的类型。

这是因为 Rust 中的 **Deref trait** 起了作用。

### Deref Trait 的作用

在 Rust 中，`Deref` trait 允许智能指针（如 `Box<T>`、`Rc<T>`、`Arc<T>` 等）在某些情况下表现得像普通的引用（`&T`）一样。当你在函数调用中传递一个智能指针的引用时，如果函数签名需要一个普通引用（`&T`），Rust 编译器会自动调用 `Deref` trait 来将智能指针的引用解引用（dereference）为它所指向的内部数据的引用。这种行为被称为**Deref 强制转换** (Deref coercion)。

```rust
fn greetings(name: &str){
    println!("Welcome! {}", name);
}

fn deref_string(){
    greetings("Xiyuan Yang");
    greetings(&String::from("Xiyuan Yang"));
    let m = Box::new(String::from("Rust"));
    greetings(&m);
    // &m: &Box<String>
    // for deref trait, &Box<String> -> &String is available!
}
```

{% note primary %}

换句话说，Deref Trait 可以让程序员不用手动的调用解引用运算符，编译器在识别类型后自动的实现了这一点。

{% endnote %}

类似于如何使⽤ Deref trait 重载不可变引⽤的 * 运算符，Rust 提供了 DerefMut trait ⽤于重载可变引⽤的 * 运算符。

Rust 在发现类型和 trait 实现满⾜三种情况时会进⾏ Deref 强制转换：

1. 当 `T: Deref<Target=U>` 时从 `&T` 到 `&U` 。
2. 当 `T: DerefMut<Target=U>` 时从 `&mut T` 到 `&mut U` 。
3. 当 `T: Deref<Target=U>` 时从 `&mut T` 到 `&U` 。

### Drop Trait

在 C++ 中，使用裸指针需要手动 delete 内存，而在 Rust 中对自定义指针实现 Drop Trait 就可以实现**自动化资源的释放**。

```rust
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

```text
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

> 有点类似于析构函数。不过和析构函数不同的一点是，Rust 并不允许我们主动调⽤ Drop trait 的 drop ⽅法，因为 Rust 会保证在 main 函数的结尾自动调用 drop，重复的调用会导致 double free 错误；当我们希望在作⽤域结束之前就强制释放变量的话，我们应该使⽤的是由标准库提供的 `std::mem::drop` 函数。

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

## `Rc<T>`

### Multi Ownership

⼤部分情况下所有权是⾮常明确的：可以准确地知道哪个变量拥有某个值。然⽽，有些情况单个值可能会有多个所有者。而启动多所有权机制并且同时保证内存安全性是非常困难的，需要手动的管理每一个所有者的生命周期，否则就会导致 double free 或者悬垂指针等严重的内存泄漏问题。


在某些场景下，你可能希望让多个部分共享同一个数据，并且不确定哪个部分会最后使用它。所有权系统在这种情况下就显得过于严格了。例如：

- 图数据结构（Graph）：一个节点可能被多个其他节点引用。

- 循环引用（Cyclic References）：比如父子节点互相引用。

- 多线程共享数据：多个线程需要访问同一个不可变数据。

在这种情况下，你无法简单地通过移动所有权来解决问题，因为没有一个单一的所有者能决定何时释放数据。

例如下面的链表代码，在创建 c 的时候因为 a 的所有权已经移动到 b 的内部，作为一个新的链表，因此无法再创建一个新的公用节点的链表：

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};
fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    // error: value used here after move
}
```

Rust 和其他现代编程语言的解决方案是**引用计数**，即记录一个值的引用数量来知晓这个值是否被引用以及被引用的状态。如果某个值有 0 个引用，那就说明这个值可以被清理。在 Rust 中，引用计数是通过 `Rc` 来实现的。

```rust
use std::rc::Rc;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum List_Safe {
    Cons_Safe(i32, Rc<List_Safe>),
    Nil_Safe,
}
use crate::List::{Cons, Nil};
use crate::List_Safe::{Cons_Safe, Nil_Safe};
fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    // error: value used here after move

    // correct code
    let a = Rc::new(Cons_Safe(5, Rc::new(Cons_Safe(10, Rc::new(Nil_Safe)))));
    println!("Count: {}", Rc::strong_count(&a)); // 1
    {
        let b = Cons_Safe(3, Rc::clone(&a));
        println!("Count: {}", Rc::strong_count(&a)); // 2
        let c = Cons_Safe(4, Rc::clone(&a));
        println!("Count: {}", Rc::strong_count(&a)); // 3
    }
    println!("Count: {}", Rc::strong_count(&a)); // 1
}
```

* `use std::rc::Rc;`: 这行代码导入了 **`Rc`（Reference Counting）智能指针。`Rc` 允许多个变量共享同一个值的所有权**。它通过维护一个引用计数器来追踪有多少个指针指向数据。**当引用计数变为 0 时，数据会自动被清理。**
* `enum List_Safe { Cons_Safe(i32, Rc<List_Safe>), Nil_Safe, }`: 这个枚举和第一个类似，但它使用 `Rc<List_Safe>` 而不是 `Box`。这个改变让多个链表可以共享同一个尾部。
* `let a = Rc::new(Cons_Safe(5, ...))`: 这行代码创建了一个 `Rc` 指针，指向链表 `5 -> 10 -> Nil_Safe` 的头部。`Rc::new` 创建了第一个引用，此时该数据的引用计数为 1。
* `let b = Cons_Safe(3, Rc::clone(&a));`: 这里创建了一个新链表 `b`，它的尾部是链表 `a`。**`Rc::clone(&a)`** 是关键。它**不会对数据进行深拷贝**。相反，它创建了一个指向 `a` 所指向数据的**新指针**，并**将 `a` 的引用计数增加了 1**。现在，`a` 和 `b` 都共享着 `5 -> 10 -> Nil_Safe` 这段数据的所有权，引用计数为 2。
> 这里的 Clone 的意思应该是针对于智能指针而言的，即创建了两个独立的智能指针，虽然他们都拥有对某个堆上数据的所有权。
* `let c = Cons_Safe(4, Rc::clone(&a));`: 这行代码现在是完全有效的，因为它没有转移 `a` 的所有权。`Rc::clone` 再次被调用，又创建了一个共享指针。现在引用计数增加到了 3。`b` 和 `c` 都指向了相同的链表尾部，而数据并没有被重复复制。这使得创建**有公共尾部的多个链表**成为可能，而这在只使用所有权和 `Box` 的情况下是做不到的。

{% note primary %}

多所有权的共享和引用存在区别：

- Rust 的 Borrow Checker 是在编译时强制确定的，即在编译时就会安全地检查是否存在不安全的引用。并且借用的生命周期**严格不可以超过原先数据的生命周期**。
- Rc 的引用计数是运行时确定的，因此生命周期是在程序运行时决定的，在最后一个拥有者离开生命周期时被销毁。

{% endnote %}


## `RefCell<T>` & 内部可变性

> 内部可变性（Interior mutability）是 Rust 中的⼀个设计模式，它允许你即使在有不可变引⽤时也可以改变数据，这通常是借⽤规则所不允许的。

`RefCell` 是 Rust 标准库中的一个智能指针，它提供了**内部可变性（Interior Mutability）** `RefCell` 并没有打破这些规则，而是将**可变性检查从编译时推迟到了运行时**。

### `RefCell`

`RefCell<T>` 内部有一个计数器，用于追踪当前有多少个借用。它提供了两个主要方法：

  * `borrow()`：获取一个**不可变**的借用。它会增加内部的不可变借用计数。
  * `borrow_mut()`：获取一个**可变**的借用。它会检查内部的可变借用计数是否为零，并且不可变借用计数也必须为零。如果检查通过，它会将可变借用计数设为 1。
  * 这一点其实和 Rust 的 Borrow Checker 的规则是如出一辙的，只不过是把这个过程换到了运行时去执行。

如果这些运行时检查失败，比如你试图在已有不可变借用时获取可变借用，或者试图在已有可变借用时再次获取任何借用，程序就会 panic。因为这些检查发生在运行时，所以 `RefCell` 有一些性能开销。

### Why `RefCell`？

`RefCell` 主要用于解决一些特殊情况，尤其是当编译时无法确定所有权和借用规则时。例如`Rc<T>` 只能提供不可变的共享所有权。当多个部分需要共享同一份数据并且需要修改它时，`RefCell` 就是必要的。

实际情况中，这两者经常结合着一起出现：如果有⼀个储存了 `RefCell<T>` 的 `Rc<T>` 的话，就可以得到有多个所有者并且可以修改的值。

{% note primary %}

实际上，这也是 RefCell 中**内部可变性**设计模式的体现！

- Rc 提供引用计数和多所有权问题的解决方案。
- RefCell 在保证不可变性的同时又给出了修改（可变性）的权限：
    - Rc 指针本身是不可变的，不可以通过 Rc 指针直接修改对应的变量
    - 但是使用 borrow_mut 方法可以实现指向内存的修改
{% endnote %}

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 创建一个可变数据，并用 Rc 和 RefCell 包装
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    
    // 创建一个新的共享所有者
    let data_cloned = Rc::clone(&shared_data);

    println!("初始数据: {:?}", shared_data.borrow());
    
    // 通过 Rc::clone 获得的不可变引用来修改内部数据
    // borrow_mut() 在运行时检查是否已经有其他可变借用
    // 如果有，会 panic
    shared_data.borrow_mut().push(4);
    // codes below are the same:
    // let mut x = shared_data.borrow_mut();
    // (*x).push(4);

    // 两个 Rc 指针都看到了修改
    println!("修改后数据: {:?}", shared_data.borrow());
    println!("克隆后数据: {:?}", data_cloned.borrow());
}
```

> 在某些设计模式中，一个对象需要被多个观察者共享，并且这些观察者可能会修改该对象的状态。`RefCell` 使得这种设计成为可能，同时确保内存安全。


## Weak

上述的引用计数的方法看似出色的解决了堆内存上的内存泄漏问题，但是事实并非如此：

理论上，我们可以创建两个 List 互相指向对方，这样这两个引用计数的值就永远不会归零，导致堆内存的内存泄漏。

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}
```


在 Rust 中，`Weak` 是一种智能指针，它和 `Rc`（引用计数）一起工作，但又和它有根本的区别。`Rc` 代表的是**强引用**（strong reference），它拥有数据的所有权，并且会增加引用计数。只要有一个 `Rc` 指针存在，它指向的数据就不会被释放。相比之下，`Weak` 代表的是**弱引用**（weak reference）。`Weak` 指针不拥有数据的所有权，也不会增加 `Rc` 的引用计数。这意味着 `Weak` 指针的存在并不会阻止数据被释放。


`Weak` 最大的用处是**解决 `Rc` 导致的循环引用**问题。

当两个或多个 `Rc` 指针相互引用，形成一个闭环时，它们的引用计数永远不会降为零。这会导致内存泄漏，因为这些数据永远不会被清理。通过使用 `Weak`，你可以打破这个循环。在循环中的某个地方，将一个 `Rc` 引用替换为 `Weak` 引用。因为 `Weak` 不增加引用计数，当所有 `Rc` 引用都超出作用域后，数据就会被正确释放，即使还有 `Weak` 指针存在。

`Weak` 本身并不能直接访问数据。它提供了两种主要方法：

1.  **`downgrade`**: 这个方法从一个 `Rc` 智能指针创建一个 `Weak` 智能指针。
2.  **`upgrade`**: 这个方法是 `Weak` 指针的“救生索”。它尝试将 `Weak` 升级为 `Rc`。
    * 如果数据仍然存在（即引用计数大于 0），`upgrade` 会成功，并返回一个 `Some(Rc<T>)`，同时增加引用计数。
    * 如果数据已经被释放（即引用计数为 0），`upgrade` 会失败，并返回 `None`。
