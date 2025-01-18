---
cls: MTNote
uid: bc5b9895-f41d-4860-b2ae-0253f775ce01
title: Rust 字符串示例
---

# Rust 字符串示例

### 从字面量构造

```rust
// 从字面量构造
fn from_literal() {
    let message = String::from("Hello");
    println!("{}", message);

    let count = message.chars().count();    // 字符数量
    println!("{}", count);
}
```

