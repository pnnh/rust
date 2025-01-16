use std::collections::HashMap;

// 创建HashMap及插入数据
fn hash_insert() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
}

// 临时变量作HashMap的键或值
// 变量所有权会发生转移
fn hash_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map2 = HashMap::new();
    map2.insert(field_name, field_value); // field_name和field_value从这一刻开始失效，若尝试使用它们则会导致编译错误
}

// 访问HashMap的指定元素
fn hash_entry() {
    let mut map3 = HashMap::new();
    map3.insert(String::from("Blue"), 10);
    map3.insert(String::from("Yellow"), 50);

    map3.entry(String::from("Red")).or_insert(50);// 如果键不存在则插入50

    let team_name = String::from("Blue");
    let score = map3.get(&team_name); // 获取键对应的值，返回Option<&V>
    println!("{:?}", score);// Some(10)

    // 打印具体的值
    match score {
        Some(value) => println!("{}: {}", team_name, value),
        None => println!("{}: Not found", team_name),
    }
}

// 遍历HashMap
fn hash_iter() {
    let mut map3 = HashMap::new();
    map3.insert(String::from("Blue"), 10);
    map3.insert(String::from("Yellow"), 50);

    for (key, value) in &map3 {
        println!("{}:{}", key, value);
    }
    println!("{:?}", map3);
}

// 通过HashMap统计单词出现次数
fn hash_word_count() {
    let text = "hello world wonderful world Therefore it is scoped under Solution\
     Unlike languages like Java, Rust does not magically resolve things that are part of the current class";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    hash_insert();
    hash_ownership();
    hash_entry();
    hash_iter();
    hash_word_count();
}
