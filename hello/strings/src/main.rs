
// 从字面量构造
fn from_literal() {
    let message = String::from("Hello");
    println!("{}", message);

    let count = message.chars().count();    // 字符数量
    println!("{}", count);
}

fn main() {
    from_literal();
}
