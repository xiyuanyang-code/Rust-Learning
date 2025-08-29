fn find_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    println!("Hello world!");
    let long_string = String::from("hello world");
    let result;
    {
        let short_string = String::from("test");
        result = find_longest(&long_string, &short_string);
        println!("The result is {}", result);
    }
    // after the scope, result is never used
    // println!("The result is {}", result);

    let novel = String::from("test string");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
}
