fn main() {
    println!("Hello world!");
    let s = String::from("hello world");
    println!("{}", first_word(&s));
    
    let s_1 = &s[1..=3]; // [1,3]
    let s_2 = &s[1..3];  // [1,3)
    println!("{s_1}");
    println!("{s_2}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
    // 可以从一个不可变引用借用更多不可变引用，但最终指向的堆内存数据都是相同的。
}
