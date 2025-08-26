use std::collections::HashMap;
fn main() {
    println!("Hello world!");
    test_vector();
    test_vector_with_different_types();
    test_string();
    test_hashmap();
}

fn test_vector() {
    println!("Test the usage of vector!");
    // create a new vector
    let new_vec_test: Vec<i32> = Vec::new();
    println!("{new_vec_test:?}");
    // or we can just use macro
    let new_vec_test = vec![1, 2, 3];
    println!("{new_vec_test:?}");

    // pushing elements for mutable vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(22);
    v.push(224);
    for i in 1..=100 {
        v.push(i * i);
    }

    println!("{v:?}");

    // reading elements from vectors
    // using borrowing
    // method1: using [] slices
    let first_value: &i32 = &v[0];
    println!("{}", first_value);

    // method2: using .get method, returning None if index out of bound
    let first_value: Option<&i32> = v.get(2);
    match first_value {
        Some(third) => {
            println!("{}", third);
        }
        None => println!("None"),
    }

    // ! after getting the reference, we cannot modify the vector in case of data race

    // traverse elements in vector
    for i in &v {
        // borrowing
        println!("{i}");
    }

    // advanced usage
    for (index, value) in v.iter().enumerate() {
        println!("Index: {index} with value: {value}");
    }

    // or modifying them in a mut vector!
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{i}");
        // ! attention, while modifying the value with a mutable reference, * (dereference operation) is required.
        *i += 50;
        println!("{i}");
    }
}

fn test_vector_with_different_types() {
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.444),
        SpreadSheetCell::Text(String::from("hello world")),
    ];
    println!("{row:?}");
    for i in &row {
        println!("{i:?}");
    }
}

fn test_string() {
    // create a new string
    let string_new = String::from("Hello world");
    println!("{}", string_new);
    let string_new = "Hello world".to_string();
    println!("{}", string_new);

    // support unicode
    let emoji_string = String::from("✅");
    println!("{}", emoji_string);

    // using push method to modify it
    let mut s = String::from("test");
    println!("{}", s);

    // function: pub fn push_str(&mut self, string: &str)
    s.push_str("wow");
    s.push_str(&String::from("hello world"));
    println!("{}", s);

    // concat string
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1 + &s2;
    // ! now s1 can not be used
    // function: fn add(self, s: &str) -> String
    // String can be transformed into &str, with Deref Coercion
    println!("{}", s3);

    // or using format macro
    let s4 = String::from("Hello!");
    let concat_s = format!("{s3}-{s2}-{s4}");
    println!("{concat_s}");
}

fn test_hashmap() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    // the ownership will be removed in the inserting process
    scores.insert(String::from("Hello world"), 33);
    scores.insert(String::from("Lili"), 150);
    scores.insert(String::from("What the fuck"), 300);
    let team_name = String::from("Hello world");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // .get function: pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    // for the get function, it will return Option<&i32>
    // unwrap_or(): handle None
    println!("{}", score);

    // support traversing
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // advanced API: entry
    // if value exists, don't do anything, else insert with the value
    use_entry();
    use_if_else();
    use_if_let();
    demo();
}

fn use_entry() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team_to_check = String::from("Yellow");

    scores.entry(team_to_check).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(20);

    println!("{:?}", scores);
}

fn use_if_else() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team_to_check = String::from("Yellow");

    if scores.get(&team_to_check).is_none() {
        scores.insert(team_to_check, 50);
    }

    let blue_team = String::from("Blue");
    if scores.get(&blue_team).is_none() {
        scores.insert(blue_team, 20);
    }

    println!("{:?}", scores);
}

fn use_if_let() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team_to_check = String::from("Yellow");

    if let None = scores.get(&team_to_check) {
        scores.insert(team_to_check, 50);
    }

    let blue_team = String::from("Blue");
    if let None = scores.get(&blue_team) {
        scores.insert(blue_team, 20);
    }

    println!("{:?}", scores);
}

fn demo() {
    let text = "Hello world wonderful world";
    let mut map: HashMap<String, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(String::from(word)).or_insert(0);
        // pub fn or_insert(self, default: V) -> &'a mut V
        // thus count is a immutable reference!
        *count += 1;
    }
    println!("{map:?}");
}
