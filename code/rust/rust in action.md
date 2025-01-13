异步图书
www.epubit.com

《Rust In Action》
《Rust实战》
新西兰 蒂姆·麦克纳马拉
2022.9
ISBN：978-7-115-59139-5

# 前言

**Rust是—门运行速度极快，能防止出现段错误并能保证线程安全的系统编程语言。**
**Rust是一门赋予每个人构建可靠且高效软件能力的语言。**

系统编程概念
计算机工作原理

目的：编写更安全软件

作为—门系统编程语言，Rust与众不同的一个特点就是，它能够在编译时就防止对无效数据的访问。

微软安全响应中心的研究项目和Chromium测览器项目都表明了，与无效数据访问相关的问题约占全部严重级安全漏洞（serious security bug）的70％。

Rust消除了此类漏洞。它能保证程序是内存安全（memory safe）的，并且不会引人额外的运行时开销。
其他语言可以提供这种级别的安全性（safety），但它们需要在程序的执行期添加额外检查，这无疑会减慢程序的运行速度。Rust设法突破了这种持续已久的状况，开辟出了属于自己的空间。

总结：
> Rust兼具安全性和可控性，其它语言则倾向于在这两者之间进行权衡利取舍。



Rust相关概念
所有权系统
Trait
包管理
错误处理
条件编译
内存模型
文件操作
多线程
网络编程


仅仅知道语法和语义是不够的，实践重要。
**社区的力量**

**创新性和主动性**
# 入门

## hello world
```shell
cargo new hello
cd hello
cargo run

输出：
warning: `\.cargo\config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
    Creating binary (application) `rust-hello` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

warning: `\.cargo\config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
   Compiling rust-hello v0.1.0 (D:\workspace\liveforever\code\sourcecode\rust-hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.13s
     Running `target\debug\rust-hello.exe`
Hello, world!
```

## cargo
构建工具、包管理器。
cargo将Rust代码转换成可执行的二进制文件，同时能够管理项目依赖包的下载和编译的过程。

```shell
cargo -V

warning: `\.cargo\config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
cargo 1.78.0 (54d8815d0 2024-03-26)
```

`cargo new`会遵照标准模板创建一个项目
```shell
D:\workspace\liveforever\code\sourcecode>tree rust-hello /F
卷 Data 的文件夹 PATH 列表
卷序列号为 807E-A390
D:\WORKSPACE\LIVEFOREVER\CODE\SOURCECODE\RUST-HELLO
│  Cargo.lock
│  Cargo.toml
│
├─src
│      main.rs
│
└─target
    │  .rustc_info.json
    │  CACHEDIR.TAG
    │
    └─debug
        │  .cargo-lock
        │  rust-hello.d
        │  rust-hello.exe
        │
        ├─.fingerprint
        │  └─rust-hello-c6c7896b1dfa3ef5
        │          bin-rust-hello
        │          bin-rust-hello.json
        │          dep-bin-rust-hello
        │          invoked.timestamp
        │
        ├─build
        ├─deps
        │      rust_hello-c6c7896b1dfa3ef5.d
        │      rust_hello-c6c7896b1dfa3ef5.exe
        │
        ├─examples
        └─incremental
            └─rust_hello-2n5xaanvgshky
                │  s-h3my2ynki8-445gi9.lock
                │
                └─s-h3my2ynki8-445gi9-dtc7bq9mf6tyyhuugkbd2bork
                        1hvah9ufeoabd9di.o
                        2b5yhhtcv4u8pn9j.o
                        31bjlb46jzd8c2ap.o
                        412613tlh6moo10a.o
                        49dmumq7fr5jv9oj.o
                        dep-graph.bin
                        l1ioelf6kj4wltt.o
                        query-cache.bin
                        work-products.bin
```

## Cargo.toml
Cargo.toml文件描述了项目的元数据，例如项目的名称、项目的版本号及其依赖项。
源代码保存在`src`。
Rust源文件使用`.rs`作为它的文件扩展名。

`cargo run`
默认使用调试模式（debug mode）来编译代码，这样可以提供最大化的错误信息（error information）。

`cargo run --release`
编译出一个发布构建的版本。
发布构建在运行时要快得多，但是需要更长的编译期。

`cargo run -q --release`
进—步减少输出信息。`q是quiet的缩写`。


`Cargo.lock`文件和target目录都是由cargo管理的。都是编译过程中的产物。

`Cargo.lock`文件指定了所有依赖项的具体版本号，所以可以保证程序总会使用同样的方式可靠地构建。直到`Cargo.toml`的内容被修改才会改变。


# 表达式
隐式返回：
> Rust提供了return关键字，但通常情况下会将其省略。

**Rust是一门基于表达式的语言。**



# 基础语法


## Vec（动态数组）
`vector`的缩写
是一个可以动态扩展的集合类型。

`Vec<_>`表示要求Rust推断出此动态数组的元素类型。

## if let结构

`parse()`方法：
> 成功解析字符串，返回`Ok<T>`（这里的T代表任何类型）
> 如果解析失败，返回`Err<E>`（这里的E代表一个错误类型）

**`if let Ok<T>`的效果就是忽略任何错误的情况。**


# 数据类型

# 错误处理

`eprintln!`输出到标准错误。
`println!`输出到标准输出。

占位符`{}`表示Rust应该使用程序员定义的方法，将该值表示为一个字符串。
`{:?}`表示要求使用该值的默认表示形式。




# 生命周期、所有权和借用



# 宏
可以把宏看作—类奇特的函数，提供了避免“样板代码” （boilerplate code）的能力。
返回的是代码（函数）而不是值。
通常，宏用于简化常见的代码模式。


`println!`宏，实际上它在底层进行了大量的类型检测工作，所以才能把任意的数据类型输出至屏幕上。


# 流程控制

for循环
continue关键字

# 方法
Rust不是面向对象的，因为它不支持继承。但是Rust用到了面向对象语言里的方法语法。

函数可以接收和返回函数



# 匿名函数（lambda函数）

# 闭包（closure）


# 类型注解
作为给编译器的提示信息。

# 条件编译


# 系统编程


# 实际运用

## 芒德布罗集（Mandelbrot set）渲染器
## 一个grep的克隆
## CPU模拟器
## 自动生成艺术项目
## 一个数据库
## HTTP、NTP以及hexdump客户端
## LOGO语官解释器
## 操作系统内核