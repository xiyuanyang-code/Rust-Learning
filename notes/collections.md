# Collections in Rust

Rust 因为其复杂的所有权机制，因此对于开发者来说在构建数据结构的时候引入了复杂性（作为保证安全性和性能的牺牲）。这也意味着在设计数据结构时，Rust 需要考虑更多东西：

- 谁拥有数据？

- 如何安全地在不同部分的代码之间共享数据？

例如，在 Rust 中创建一个双向链表（使用裸指针）会比在 C++ 中复杂得多。因为在 C++ 中，你可以简单地使用裸指针来相互引用，但在 Rust 中，这种裸指针的使用受到严格限制。你需要使用 Rc（引用计数）或 RefCell（运行时借用检查）等智能指针，才能实现安全的循环引用。

同时，Rust 标准库内置的复合类型的数据结构无法满足现实生活复杂多变的需求。

- 数组：在栈上分配内存，储存相同类型的元素，并且**大小在编译时确定不可变**。
- 元组：在栈上分配内存，储存不同类型的元组，**大小在编译时确定不可变**。

因此，本文将介绍 **Collections in Rust**，作为标准库提供更多功能更加丰富的数据结构类型。

- Vector
- String
- HashMap

## Vector

类型：`vec<T>`，使用方式和接口完全类似于 C++ STL 中的 vector，**允许可变长度的数组 & 只能存储相同类型的引用**。

对于不同类型存储的问题，可以使用**枚举**的方式解决。相当于自定义一个嵌套类型。

```rust
fn main() {
    println!("Hello world!");
    test_vector();
    test_vector_with_different_types();
}

fn test_vector() {
    println!("Test the usage of vector!");
    // create a new vector
    let new_vec_test: Vec<i32> = Vec::new();
    println!("{new_vec_test:?}");
    // or we can just use macro
    let new_vec_test = vec![1, 2, 3];
    println!("{new_vec_test:?}");

    // pushing elements for mutable vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(22);
    v.push(224);
    for i in 1..=100 {
        v.push(i * i);
    }

    println!("{v:?}");

    // reading elements from vectors
    // using borrowing
    // method1: using [] slices
    let first_value: &i32 = &v[0];
    println!("{}", first_value);

    // method2: using .get method, returning None if index out of bound
    let first_value: Option<&i32> = v.get(2);
    match first_value {
        Some(third) => {
            println!("{}", third);
        }
        None => println!("None"),
    }

    // ! after getting the reference, we cannot modify the vector in case of data race

    // traverse elements in vector
    for i in &v {
        // borrowing
        println!("{i}");
    }

    // advanced usage
    for (index, value) in v.iter().enumerate() {
        println!("Index: {index} with value: {value}");
    }

    // or modifying them in a mut vector!
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{i}");
        // ! attention, while modifying the value with a mutable reference, * (dereference operation) is required.
        *i += 50;
        println!("{i}");
    }
}

fn test_vector_with_different_types() {
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.444),
        SpreadSheetCell::Text(String::from("hello world")),
    ];
    println!("{row:?}");
    for i in &row {
        println!("{i:?}");
    }
}
```

## String

```rust
fn test_string(){
    // create a new string
    let string_new = String::from("Hello world");
    println!("{}", string_new);
    let string_new = "Hello world".to_string();
    println!("{}", string_new);

    // support unicode
    let emoji_string = String::from("✅");
    println!("{}", emoji_string);

    // using push method to modify it
    let mut s = String::from("test");
    println!("{}",s);

    // function: pub fn push_str(&mut self, string: &str)
    s.push_str("wow");
    s.push_str(&String::from("hello world"));
    println!("{}",s);

    // concat string
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1 + &s2;
    // ! now s1 can not be used
    // function: fn add(self, s: &str) -> String
    // String can be transformed into &str, with Deref Coercion
    println!("{}",s3);

    // or using format macro
    let s4 = String::from("Hello!");
    let concat_s = format!("{s3}-{s2}-{s4}");
    println!("{concat_s}");
}
```

### Index String

在 Rust 中，直接对 String 字符串使用索引是非法的。我们先从 String 的内部原理讲起。

String 是一个 `Vec<u8>` 的封装。在像 C 或 C++ 这样的语言中，字符串通常由一个字节（char）数组组成，每个字符都占用一个固定的字节大小（通常为 1 字节）。这种情况下，要访问第 n 个字符，只需简单地计算 n * sizeof(char) 即可，因此可以直接索引。然而，Rust 的 `String` 和 `&str` 类型使用的是 UTF-8 编码。在 UTF-8 中：

- 一个字符（unicode 标量值）可能由 1 到 4 个字节组成。

- 一个字符的可视外观（字形簇或 grapheme cluster）可能由一个或多个 unicode 标量值组成。例如，"é" 是一个字符，但它可以由 e 和一个重音符号的 unicode 组合而成。

由于字符的字节长度不固定，直接通过索引访问会变得复杂且不安全：

- s[0] 是什么？ 如果你直接索引到 s[0]，这到底是第 1 个字节、第 1 个 unicode 字符，还是第 1 个可视字符？这些在 UTF-8 中是不同的概念。

- 性能问题：如果直接索引到 s[i]，编译器不知道第 i 个字符的字节偏移量，它必须从字符串的开头开始，逐个字节地遍历，直到找到第 i 个字符。这会使得索引操作不再是 O(1) 时间复杂度，而是 O(n)。 


Rust 通过提供三种不同的视图来安全地处理字符串，让你明确自己想要遍历的是什么：

1.  **按字节遍历 (`.bytes()`)**：如果你需要处理底层的字节数据，可以使用 `.bytes()` 方法。这会返回一个迭代器，每次迭代会得到一个 `u8` 类型的值。

    ```rust
    let s = "你好";
    for b in s.bytes() {
        println!("{}", b);
    }
    // 输出：
    // 228
    // 189
    // 160
    // ...
    ```

2.  **按 Unicode 标量值遍历 (`.chars()`)**：如果你想处理单个的 unicode 字符，可以使用 `.chars()` 方法。这会返回一个迭代器，每次迭代会得到一个 `char` 类型的值。

    ```rust
    let s = "你好吗";
    for c in s.chars() {
        println!("{}", c);
    }
    // 输出：
    // 你
    // 好
    // 吗
    ```

## HashMap

`HashMap<K,V>`

> 有关哈希函数和哈希表的具体使用方式在此处不做介绍，这里只介绍 Rust 独有的语言特性和语法。

```rust
fn test_hashmap() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    // the ownership will be removed in the inserting process
    scores.insert(String::from("Hello world"), 33);
    scores.insert(String::from("Lili"), 150);
    scores.insert(String::from("What the fuck"), 300);
    let team_name = String::from("Hello world");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // .get function: pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    // for the get function, it will return Option<&i32>
    // unwrap_or(): handle None
    println!("{}", score);

    // support traversing
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // advanced API: entry
    // if value exists, don't do anything, else insert with the value
    use_entry();
    use_if_else();
    use_if_let();
    demo();
}
```

### `entry`

常见操作：查找键是否存在，如果不存在则插入对应的 value，如果存在就不做任何事。

```rust
fn use_entry() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team_to_check = String::from("Yellow");

    scores.entry(team_to_check).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(20);

    println!("{:?}", scores);
}

fn use_if_else() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team_to_check = String::from("Yellow");

    if scores.get(&team_to_check).is_none() {
        scores.insert(team_to_check, 50);
    }
    
    let blue_team = String::from("Blue");
    if scores.get(&blue_team).is_none() {
        scores.insert(blue_team, 20);
    }

    println!("{:?}", scores);
}

fn use_if_let() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team_to_check = String::from("Yellow");

    if let None = scores.get(&team_to_check) {
        scores.insert(team_to_check, 50);
    }
    
    let blue_team = String::from("Blue");
    if let None = scores.get(&blue_team) {
        scores.insert(blue_team, 20);
    }

    println!("{:?}", scores);
}
```

### Demo

可以使用 HashMap 来自动储存每一个单词的出现的次数。

```rust
let text = "Hello world wonderful world";
let mut map: HashMap<String, i32> = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(String::from(word)).or_insert(0);
    // pub fn or_insert(self, default: V) -> &'a mut V
    // thus count is a immutable reference!
    *count += 1;
}
println!("{map:?}");
```

* `map.entry(...)`：这是 `HashMap` 的一个高级方法，它会查找给定的键。它返回一个 `Entry` 枚举，代表了两种可能的状态：
    * **键已存在** (`Occupied`)：如果单词已经在 `map` 中，`entry` 会返回一个指向该键值对的引用。
    * **键不存在** (`Vacant`)：如果单词是第一次出现，`entry` 会返回一个空缺的引用。
* `.or_insert(0)`：这是一个链式调用，它处理 `Entry` 的两种状态：
    * 如果键已存在，它什么也不做，并返回**指向现有值的可变引用**。
    * 如果键不存在，它会插入一个值为 `0` 的新键值对，并返回**指向这个新值 `0` 的可变引用**。
* `let count = ...`：因此，无论单词是第一次出现还是已经存在，`count` 变量都会是一个类型为 `&mut i32` 的**可变引用**，指向 `map` 中对应的值。

> 使用 entry 方法可以避免两次哈希表查找（一次 find，一次 insert），从而优化时间效率，并且代码更加美观。
> It is more Rusty! 

{% note primary %}

Functional Programming 简直是无处不在。

{% endnote %}


## More Collections...

| 类别 | 数据结构 | 描述 |
| :--- | :--- | :--- |
| **序列 (Sequences)** | `Vec<T>` | 最常用的可变长动态数组。元素在内存中连续存储，支持快速随机访问。 |
| | `VecDeque<T>` | 双端队列。支持在队列的头部和尾部高效地插入和删除元素。 |
| | `LinkedList<T>` | 双向链表。在序列中间进行插入和删除操作时性能很高。 |
| **映射 (Maps)** | `HashMap<K, V>` | 基于哈希表的键值对集合。查找、插入和删除操作的平均时间复杂度为 O(1)。 |
| | `BTreeMap<K, V>` | 基于 B-Tree 的有序键值对集合。键始终保持排序，遍历时有序。 |
| **集合 (Sets)** | `HashSet<T>` | 基于哈希表的集合，存储唯一的元素。操作的平均时间复杂度为 O(1)。 |
| | `BTreeSet<T>` | 基于 B-Tree 的有序集合，存储唯一的元素。遍历时按值排序。 |
| **堆 (Heap)** | `BinaryHeap<T>` | 最大二叉堆。常用于实现优先队列，可以高效地获取和删除最大元素。 |