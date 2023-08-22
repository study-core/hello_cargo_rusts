# Cargo 及 目录

## 前言

Cargo.toml 和 Cargo.lock 是 Cargo 的两个元配置文件，但是它们拥有不同的目的:

前者从用户的角度出发来描述项目信息和依赖管理，因此它是由用户来编写
后者包含了依赖的精确描述信息，它是由 Cargo 自行维护，因此不要去手动修改
它们的关系跟 package.json 和 package-lock.json 非常相似，从 JavaScript 过来的同学应该会比较好理解。


## 工作空间

Cargo 提供了一个叫 工作空间（workspaces）的功能，它可以帮助我们管理多个相关的协同开发的包。

工作空间 是一系列共享同样的 Cargo.lock 和输出目录的包。

## 创建工作空间

如：
我们的工作空间有一个 二进制项目 (main) crate 和 两个库 (lib) crate。

二进制项目会提供主要功能，并会依赖另两个库。
一个库会提供 add_one 方法
而第二个会提供 add_two 方法。
这三个 crate 将会是相同工作空间的一部分。

1. **让我们以新建工作空间目录开始：**



```bash
mkdir adds_workspace
cd adds_workspace
```

2. **创建工作空间配置：**

接着在 `adds_workspace` 目录中，创建 Cargo.toml 文件。

这个 Cargo.toml 文件配置了整个工作空间。

它不会包含 [package] 部分。

相反，它以 [workspace] 部分作为开始，并通过指定 adder (main crate)  的路径来为工作空间增加成员，如下会加入二进制 crate：

```toml
// Cargo.toml

[workspace]

members = [
    "adder",  // 这个是 main crate
]

```

3. **创建 adder (main crate) ：**

接下来，在 `adds_workspace` 目录运行 `cargo new` 新建 adder 二进制 crate：

```bash

cargo new adder
//     Created binary (application) `adder` package

```

4. **创建工作空间：**

到此为止，可以运行 `cargo build` 来构建工作空间。

`adds_workspace` 目录中的文件应该看起来像这样：

```
├── Cargo.lock              // 依赖的精确描述信息，它是由 Cargo 自行维护
├── Cargo.toml              // 工作空间中的 依赖管理
├── adder                   // adder (main crate) 目录
│   ├── Cargo.toml          // adder 的 依赖管理
│   └── src                 // adder 的 src
│       └── main.rs         // adder 的 入口 (crate： main)
└── target                  // 工作空间中所有 crate 的编译结果目录 (即使进入 adder 目录运行 cargo build，构建结果也位于 add/target 而不是 add/adder/target。工作空间中的 crate 之间相互依赖)

// 通过共享一个 target 目录，工作空间可以避免其他 crate 多余的重复构建

```

5. **创建第二个包：**

类似 adder 去创建新目录 add_one ，并修改顶级 Cargo.toml 为也包含 add_one 路径：

```toml
// Cargo.toml

[workspace]

members = [
    "adder",        // 这个是 main crate
    "add_one",      // 这个是 lib crate
]


```

6. **创建 add_one (lib crate) ：**

接下来，在 `adds_workspace` 目录运行 `cargo new` 新建 add_one 库 crate：

```bash

cargo new add_one --lib
//     Created library `add_one` package

```

现在 add 目录应该有如下目录和文件：


```
├── Cargo.lock
├── Cargo.toml
├── add_one                     // add_one (lib crate) 目录
│   ├── Cargo.toml              // add_one 的 依赖管理
│   └── src                     // add_one 的 src
│       └── lib.rs              // add_one 的 入口 (crate： lib)
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target

```


1. **编辑 lib.rs**

在 add_one/src/lib.rs 文件中，增加一个 add_one 函数：

```rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

8. **为 adder 添加 add_one 依赖：**

现在我们有了二进制 adder 依赖库 crate add_one。首先需要在 adder/Cargo.toml 文件中增加 add_one 作为路径依赖：

```toml
// adder/Cargo.toml

[dependencies]
add_one = { path = "../add_one" }

```


9. **adder 使用 add_one 库：**

打开 adder/src/main.rs 在顶部增加一行 use 将新 add_one 库 crate 引入作用域。接着修改 main 函数来调用 add_one 函数：

```rs
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}

```

10. **构建工作空间：**

在 `adds_workspace` 目录中运行 `cargo build` 来构建工作空间！

```bash
cargo build
//   Compiling add_one v0.1.0 (file:///projects/adds_workspace/add_one)
//   Compiling adder v0.1.0 (file:///projects/adds_workspace/adder)
//   Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

11. **在顶层 adds_workspace 目录运行二进制 crate**

```bash
cargo run -p adder
//    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
//    Running `target/debug/adder`
// Hello, world! 10 plus one is 11!


```

## 在工作空间依赖外部包

工作空间只在根目录有一个 Cargo.lock，而不是在每一个 crate 目录都有 Cargo.lock。

这确保了所有的 crate 都使用完全相同版本的依赖。

如果在工作空间的 Cargo.toml 和 add_one/Cargo.toml 中都增加 rand crate，则 Cargo 会将其都解析为同一版本并记录到唯一的 Cargo.lock 中。
(使得工作空间中的所有 crate 都使用相同的依赖意味着其中的 crate 都是相互兼容的)


