fn main() {
    let s1 = "Hello, world!";
    println!("String literal: {}", s1);

    let mut s2 = String::from("Rust is fun!");
    println!("String type: {}", s2);

    // 追加字符串
    s2.push_str(" Let's learn it!");
    println!("Appended string: {}", s2);
}