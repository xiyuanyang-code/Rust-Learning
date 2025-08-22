fn main() {
    let s1 = String::from("Hello world");
    println!("s1: {}", s1);
    {
        println!("Entering the scope of s2");
        let s2 = s1.clone();
        println!("s2: {}", s2);
        println!("{}", s1);
    }

}