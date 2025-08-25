# Packages, Crates and Modules in Rust

{% note primary %}

这一部分相当的复杂，笔者暂且学一个皮毛，在后续的学习过程中会不断的更新这篇文章~

{% endnote %}

使用包、crate 和模块管理不断增长的 Rust 项目：代码的模块化。

代码模块化是代码重构的核心，主要包含以下几点：

- 复杂性管理

- 定义清晰的公有接口实现代码复用

- 多文件的模块化

## 包和 crate

- crate：Rust 的最小编译单位。一个 crate 往往包含着一个或者多个文件。

    - 二进制 crate：包含 main 函数的 crate
    - 库 crate：提供复用模块（库）
    - crate root 是编译的起始点
        - 对于 binary crate：crate root 默认为 `src/main.rs`
        - 对于 lib crate：crate root 默认为 `src/lib.rs`

- packages：提供一系列功能的多个（或者一个）crate 的捆绑。**一个包会包含 `Cargo.toml` 文件**，阐述如何去构建这些 crate。
    - 包中可以包含最多一个库 crate
    - 但是包中可以包含任意个二进制的 crate
    - 但是包中至少包含一个 crate

## What will the compiler do?

先来简单的介绍一下编译器在 编译一次 crate 的时候会发生什么事情：

{% note primary %}

### 识别 `crate` 根

首先，编译器会根据你提供的命令或 Cargo 的设置，找到 **`crate` 的根文件**。

* 如果是二进制 `crate`，它会寻找 `src/main.rs`。
* 如果是库 `crate`，它会寻找 `src/lib.rs`。

这个根文件是编译器的“起点”，它告诉编译器整个 `crate` 的入口在哪里。

### 构建模块树 (Module Tree)

接下来，编译器会从 `crate` 根开始，根据 `mod` 关键字来**递归地遍历和解析**整个 `crate` 的所有文件。

* 当它在 `src/main.rs` 或 `src/lib.rs` 中看到 `mod my_module;` 这行时，它就知道要去寻找 `src/my_module.rs` 或 `src/my_module/mod.rs` 这个文件。
* 如果 `my_module.rs` 里又包含 `mod another_module;`，编译器会继续寻找 `src/another_module.rs`。
* 也有可能之间内联在该文件中， Rust 也会自动识别这一点。

这个过程会一直持续，直到编译器将所有相关的 `.rs` 文件都组织成一个完整的**模块树**。这个树状结构清晰地定义了每个模块之间的父子关系和层次结构。

### 检查可见性 (Visibility Checks)

在构建模块树后，编译器会进行可见性检查。它会：

* 检查每个**路径**（例如 `my_module::some_function`）是否合法。
* 验证你调用的函数、结构体或枚举是否是 `pub`（公开）的，并且在当前作用域中可以访问。
* 如果一个模块或项没有被标记为 `pub`，编译器会阻止外部模块对它的访问，并抛出一个编译错误。

这个步骤确保了你的代码遵循 Rust 的**私有性**和**封装**规则，防止意外的外部访问。
> 在 Rust 中，可见性规则是层层递进的，就像一扇扇上锁的门。要访问房间里的东西，你必须先打开所有通往那个房间的门。

### 编译和链接

最后，在所有文件都解析完毕、所有路径和可见性都检查无误后，编译器会开始将所有 `.rs` 文件中的代码**编译成目标代码**（例如 `.o` 文件）。

* 它会单独编译每个模块，但同时会根据模块树的依赖关系来处理。
* 编译完成后，它会将所有生成的**目标代码**和必要的**库代码**（来自标准库或其他依赖）**链接**在一起，最终生成一个单一的可执行文件（`bin`）或库文件（`lib`）。

{% endnote %}

## 相关概念

### 模块树

模块树（Module Tree）是Rust中所有模块的层次结构，类似于文件系统的目录树。每个crate都有一个根模块，所有其他模块都是这个根模块的子模块或后代模块。

模块树是理解模块之间可见性和管理模块化代码的**核心**，他提供了一种抽象的模式来管理多个模块之间的复杂依赖。在分析模块可视性之前，使用模块树是最好并且最清晰的分析方式。

```text
crate packages_modules
├── mod a: pub(crate)
│   ├── mod b: pub
│   │   └── fn fun_b: pub
│   ├── mod c: pub(self)
│   │   └── fn fun_c: pub
│   └── fn fun_a: pub
├── mod garden: pub
│   └── mod vegetables: pub
│       └── struct Asparagus: pub
└── fn main: pub(crate)
```

### `mod`

`mod` 是 Rust 中用于**声明和组织代码模块**的关键字。它就像是你在 Rust 项目中创建文件夹，帮助你将代码按逻辑功能分组，提高代码的可读性、可维护性和复用性。

1.  **内联模块（Inline Module）**
    当模块代码量较少时，你可以直接在当前文件中使用 `mod` 关键字创建内联模块。这就像在一个文件中创建了一个私有子代码块。

    ```rust
    // main.rs
    mod my_utility {
        // 这个函数只在 my_utility 模块内部可见
        fn internal_helper() {
            println!("I'm a private helper!");
        }

        // 使用 pub 关键字，这个函数才能被外部模块访问
        pub fn do_something() {
            internal_helper(); // 模块内可以调用私有函数
            println!("Doing something useful.");
        }
    }

    fn main() {
        my_utility::do_something();
        // 错误：my_utility::internal_helper() 是私有的
    }
    ```

    在这个例子中，`my_utility` 就是一个内联模块。

2.  **文件模块（File Module）**
    这是最常见的用法，当模块代码量较大时，通常会将模块的声明和定义分开。你用 `mod` 声明模块，而实际的代码则放在一个单独的文件或文件夹中。

    ```rust
    // main.rs (crate 根)
    mod geometry; // 声明一个名为 geometry 的模块

    fn main() {
        geometry::shape::circle();
    }
    ```

    当你写下 `mod geometry;` 时，编译器会根据惯例去寻找以下两个位置之一：

      * **`src/geometry.rs`**：一个名为 `geometry.rs` 的文件，其内容即为 `geometry` 模块。
      * **`src/geometry/mod.rs`**：一个名为 `geometry` 的文件夹，其中包含一个 `mod.rs` 文件，其内容为 `geometry` 模块。

    如果 `geometry` 模块中还有子模块，比如 `shape`，那么 `src/geometry.rs` 的内容可能如下：

    ```rust
    // src/geometry.rs
    pub mod shape; // 声明 geometry 模块下的一个子模块 shape
    ```

    然后，`shape` 模块的代码会放在 **`src/geometry/shape.rs`** 或 **`src/geometry/shape/mod.rs`** 中。(按照从左到右的优先级，先查找 `src/geometry/shape.rs`)

### path

在 Rust 中，要访问模块中的项（比如函数、结构体、枚举等），你需要使用**路径**。路径就像是文件系统中的地址，它告诉编译器如何找到你想要使用的项。

#### Absolute Path

绝对路径从 `crate` 的根开始，使用关键字 `crate`。这就像是提供一个完整的、从根目录开始的地址。当你需要访问当前 `crate` 中任何位置的项时，都可以使用绝对路径。它清晰且不易出错，因为它总是从同一起点开始。

> 从模块树的角度，就是从根节点出发走向目标节点的路径。

#### Relative Path

相对路径从当前模块开始，使用 `self`、`super` 或直接使用模块名。

{% note primary %}

在 Rust 的模块树中，每个 `.rs` 文件或 `mod {}` 代码块都代表一个模块。当你在这段代码的内部编写代码时，这段代码就处于“当前模块”的作用域。

{% endnote %}

  * **`self`**: 指代当前模块本身。
  * **`super`**: 指代父模块。你可以多次使用 `super` 来向上移动。
  * **模块名**: 直接使用兄弟模块或子模块的名称。

假设你在一个名为 `a` 的模块中，它有一个子模块 `b`，还有一个兄弟模块 `c`。

```rust
mod a {
    pub fn fun_a() {}

    mod b {
        use super::fun_a; // 使用 super 访问父模块 a 中的 fun_a
        pub fn fun_b() {
            fun_a();
        }
    }

    mod c {
        pub fn fun_c() {
            // 在 c 中，如果要访问 b，需要从父模块 a 开始
            // crate::a::b::fun_b();
        }
    }
}
```

### `use` 

为了避免每次都写很长的路径，你可以使用 `use` 关键字将路径引入到当前作用域。这就像是创建了一个快捷方式。

```rust
// 不使用 use
// let plant = crate::garden::vegetables::Asparagus {};

// 使用 use
use crate::garden::vegetables::Asparagus;
let plant = Asparagus {};
```

`use` 声明通常放在文件的顶部，它使得代码更简洁，更容易阅读。你可以一次性引入多个项，或为项指定别名，以避免命名冲突。

可以使用 `as` 来指定别名防止命名冲突，和 Python 的 import 语法很像。

使⽤ use 关键字，将某个名称导⼊当前作⽤域后，该名称**对此作⽤域之外还是私有的**。若要让作⽤域之外的代码能够像在当前作⽤域中⼀样使⽤该名称，可以将 pub 与 use 组合使⽤。这种技术被称为重导出（re-exporting），因为在把某个项⽬导⼊当前作⽤域的同时，也将其暴露给其他作⽤域。


### pub

Make an item visible to others.

The keyword `pub` makes any module, function, or data structure accessible from inside of external modules. The pub keyword may also be used in a use declaration to re-export an identifier from a namespace.

## Visibility & Privacy

这一部分实在太太太复杂了，始终无法成为人型编译器。
Relevant Website:

- [Comprehensive Rust](https://google.github.io/comprehensive-rust/zh-CN/modules/visibility.html)
- [Rust Reference: Visibility](https://rustwiki.org/zh-CN/reference/visibility-and-privacy.html)

Rust 模块化的关键在于**可见性的构建**，因为 Rust 没有类，为了提供封装性，Rust 在编译过程中引入了**检查可见性**的环节。

> 检查可见性的环节在构建模块树之后，但是**Rust不会检查依赖关系**，Rust的依赖关系只和**可见性有关**，因此 Rust的编译器无法在编译过程中杜绝**循环引用**事件的发生。

下面介绍三条最重要的**Rust可见性规则**。


{% note info %}

注意，下面的讨论内容都是**默认不使用 `pub` 的行为**，因为加了 pub 就相当于门户大开全局可见了。 

{% endnote %}

### 默认情况下，模块项是私有的

```rust
mod my_module {
    fn private_function() {
        // 这个函数默认是私有的，只能在my_module内部访问
    }
    
    pub fn public_function() {
        // 使用pub关键字使其公开
    }
}

fn main() {
    // my_module::private_function(); // ❌ 错误！无法访问私有函数
    my_module::public_function();    // ✅ 正确！可以访问公共函数
}
```

### 原子操作的可见性判断

关键：从**模块树**的角度分析可见性！

理论上来说，在任何一个当前模块下，到达任何一个模块的结构（例如函数，结构体等等），理论上只需要两个原子操作：**到达父亲节点 & 到达儿子节点**。因此，我们首先分析这两个原子操作的可见性。

- 父项可见性：在对父亲项不加 pub 的情况下，使用 `super::` 关键字访问其父项。
    - 并且**可以直接访问父项的私有函数**。
    - 以此类推，`super::super::` 也是可见的，也就是说，**沿着模块树不断往上走，祖先模块的组件均可以被访问**。
    - **如果某个程序项是私有的，则当前模块及当前模块的后代模块都可以访问它**。

    ```rust
    mod grandfather {
        fn grandfather_fn() {}
        pub fn public_grandfather_fn() {}
        
        mod father {
            fn father_fn() {}
            pub fn public_father_fn() {}
            
            mod son {
                fn son_fn() {
                    // 可以访问父项
                    super::father_fn();
                    super::public_father_fn();
                    
                    // 可以访问祖父项
                    super::super::grandfather_fn();        
                    super::super::public_grandfather_fn(); 
                }
            }
        }
    }
    ```

- 儿子项可见性：**在模块内部**，在对儿子项不加 pub 的情况下，可以访问**自己的直接子模块**（哪怕儿子的 mod 没有加 pub 修饰），但是在模块外部的访问收到了 pub 的限制（儿子可以，孙子不可以！）
    - 因此，可以换一种定义**可见**的方法：如果某个程序项是公有的，那么如果可以从外部的某一模块 m 访问到该程序项的所有祖先模块，则一定可以从这个模块 m 中访问到该程序项。

```rust
mod parent {
    mod private_child {
        pub fn child_function() {}
        fn child_function_private() {}
    }
    
    pub mod public_child {
        pub fn child_function() {}
        fn child_function_private() {}
    }
    
    fn test_access() {
        private_child::child_function(); 
        // private_child::child_function_private(); ❌ 
        public_child::child_function();
        // public_child::child_function_private(); ❌
    }
}

// 在parent外部访问
fn external_access() {
    // parent::private_child::child_function();  ❌ 
    parent::public_child::child_function();   
}
```

{% note primary %}

- crate 需要一个全局可用的“辅助模块(helper module)”，但又不想将辅助模块公开为公共API。为了实现这一点，可以在整个 crate 的根模块（路径层级结构中的最顶层）下建一个私有模块，该模块在内部是“公共API”。因为整个 crate 都是根模块的后代，所以整个本地 crate 里都可以通过第二种情况访问这个私有模块。

    - 这其实和二子模块的访问是一致的，比如我希望使用一个层级比较深的模块的一个公有函数，那我需要保证这个路径下的所有都是 pub 开绿灯（不然没有访问的权限）

- 在为模块编写单元测试时，通常的习惯做法是给要测试的模块加一个命名为 mod test 的直接子模块。这个模块可以通过第二种情况访问父模块的任何程序项，这意味着内部实现细节也可以从这个子模块里进行无缝地测试。
    
    - 在 Rust 中，一个模块可以是私有的，但其内部的程序项（如函数、结构体等）可以是公有的。这在您的 crate 内部创建公共 API 时非常有用。

    - 当您在 crate 的根模块（crate root）下创建一个私有模块，并在其中定义公有项时，这个私有模块及其内部的公有项只能在当前 crate 的内部被访问。因为整个 crate 都是根模块的后代，所以 crate 内的任何地方都可以通过 crate::... 路径访问到这个私有模块。

    - 核心思想：这是一种实现内部共享代码的模式。它允许您在不将模块暴露给外部用户的情况下，在自己的 crate 内部创建可复用的“辅助模块”或共享工具，从而实现代码的组织和复用，同时保持清晰的封装边界。

- 在为模块编写单元测试时，通常的习惯做法是给要测试的模块加一个命名为 mod test 的直接子模块。这个模块可以通过第二种情况访问父模块的任何程序项，这意味着内部实现细节也可以从这个子模块里进行无缝地测试。

    - 这个测试模块利用了第二个概念：尽管 tests 是父模块的子模块，它仍然可以访问父模块的所有私有程序项。这意味着您可以在测试中直接访问和验证父模块的内部实现细节，而不仅仅是其公有 API。

{% endnote %}

```rust
// 这个模块是私有的，这意味着没有外部crate 可以访问这个模块。
// 但是，由于它在当前 crate 的根模块下，
// 因此当前 crate 中的任何模块都可以访问该模块中任何公有可见性程序项。
mod crate_helper_module {

    // 这个函数可以被当前 crate 中的任何东西使用
    pub fn crate_helper() {}

    // 此函数*不能*被用于 crate 中的任何其他模块中。它在 `crate_helper_module` 之外不可见，
    // 因此只有当前模块及其后代可以访问它。
    fn implementation_detail() {}
}

// 此函数“对根模块是公有”的，这意味着它可被链接了此crate 的其他crate 使用。
pub fn public_api() {}

// 与 'public_api' 类似，此模块是公有的，因此其他的crate 是能够看到此模块内部的。
pub mod submodule {
    use crate_helper_module;

    pub fn my_method() {
        // 本地crate 中的任何程序项都可以通过上述两个规则的组合来调用辅助模块里的公共接口。
        crate_helper_module::crate_helper();
    }

    // 此函数对任何不是 `submodule` 的后代的模块都是隐藏的
    fn my_implementation() {}

    #[cfg(test)]
    mod test {

        #[test]
        fn test_my_implementation() {
            // 因为此模块是 `submodule` 的后代，因此允许它访问 `submodule` 内部的私有项，而不会侵犯隐私权。
            super::my_implementation();
        }
    }
}

fn main() {}
```

### pub(in path), pub(crate), pub(super), and pub(self)

> Advanced Techniques...

除了公有和私有之外，Rust 还允许用户（用关键字 pub ）声明仅在给定作用域内可见的程序项。声明形式的限制规则如下：

- pub(in path) 使一个程序项在提供的 path 中可见。path 必须是声明其可见性的程序项的祖先模块。
- pub(crate) 使一个程序项在当前 crate 中可见。
- pub(super) 使一个程序项对父模块可见。这相当于 pub(in super)。
- pub(self) 使一个程序项对当前模块可见。这相当于 pub(in self) 或者根本不使用 pub。
> 从 2018版开始，pub(in path) 的路径必须以 crate、self或super开头。

```rust
pub mod outer_mod {
    pub mod inner_mod {
        // 此函数在 `outer_mod` 内部可见
        pub(in crate::outer_mod) fn outer_mod_visible_fn() {}
        // 此函数对整个 crate 都可见
        pub(crate) fn crate_visible_fn() {}

        // 此函数在 `outer_mod` 下可见
        pub(super) fn super_mod_visible_fn() {
            // 此函数之所以可用，是因为我们在同一个模块下
            inner_mod_visible_fn();
        }

        // 这个函数只在 `inner_mod` 中可见，这与它保持私有的效果是一样的。
        pub(self) fn inner_mod_visible_fn() {}
    }
    pub fn foo() {
        inner_mod::outer_mod_visible_fn();
        inner_mod::crate_visible_fn();
        inner_mod::super_mod_visible_fn();

        // 此函数不再可见，因为我们在  `inner_mod` 之外
        // 错误! `inner_mod_visible_fn` 是私有的
        //inner_mod::inner_mod_visible_fn();
    }
}

fn bar() {
    // 此函数仍可见，因为我们在同一个 crate 里
    outer_mod::inner_mod::crate_visible_fn();

    // 此函数不再可见，因为我们在`outer_mod`之外
    // 错误! `super_mod_visible_fn` 是私有的
    //outer_mod::inner_mod::super_mod_visible_fn();

    // 此函数不再可见，因为我们在`outer_mod`之外
    // 错误! `outer_mod_visible_fn` 是私有的
    //outer_mod::inner_mod::outer_mod_visible_fn();

    outer_mod::foo();
}

fn main() { bar() }
```
