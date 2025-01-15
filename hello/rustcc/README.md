# Rust调用C代码示例

本示例演示了如何在Rust中调用C代码。在macOS M1系统Rust 1.84.0版本下测试通过。

这里用C语言简单实现一个乘法函数，然后在Rust中调用该函数。

首先修改`Cargo.toml`文件，添加对`cc`和`libc`库的引用，并指定`build.rs`文件的目录。

内容如下：

```toml
[package]
name = "rustcc"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[dependencies]
libc = "0.2.169"

[build-dependencies]
cc = "1.2.9"
```

然后在项目根目录下创建`build.rs`文件，内容如下：

```rust
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/math.c")
        .compile("calculator");
}
```

`build.rs`文件用于编译C代码，将在编译期执行。这里使用`cc`库来编译`src/math.c`文件，并生成名为`calculator`的库。


接着在`src`目录下创建`math.c`文件，内容如下：

```c
#include <stdio.h>

int32_t multiply(int32_t input) {
  printf("multiply: %d\n", input);

  return input * 2;
}
```

最后在`src`目录下创建`main.rs`文件，内容如下：

```rust
extern crate libc;

#[link(name = "calculator")]
extern "C" {
    fn multiply(input: i32) -> i32;
}

fn run_calc() {
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);

    let input = 4;
    let output = unsafe {
        multiply(input)
    };
    println!("{} * 2 = {}", input, output);
}


fn main() {
    println!("Hello, world!");

    run_calc();
}
```

其中`extern "C" {}`块用于声明C函数的签名，`multiply`函数的签名与C代码中的`multiply`函数一致。
在`extern "C" {}`上面的`#[link(name = "calculator")]`用于指定链接的库名，该名称和`build.rs`中指定的库名称一致。

同时对C语言函数的调用需要包裹在`unsafe`块中。
最后在`main`函数中，调用`multiply`函数，传入相应参数，并打印结果。