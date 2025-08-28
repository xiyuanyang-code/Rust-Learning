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