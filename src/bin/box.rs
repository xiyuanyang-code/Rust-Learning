use crate::List::{Cons, Nil};
use std::any::type_name;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("Hello world!");
    intro_box();
    use_list();
    deref_trait();
    deref_string();
    deref_string_2();
}

fn print_type<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn intro_box() {
    let b = Box::new(5);
    println!("The type of b is {}", print_type(&b));
    println!("b = {b}");
}

fn use_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
}

fn deref_trait() {
    let x = 50;
    let ref_1 = &x;
    let y = Box::new(x);
    println!("{}", x == *ref_1);
    println!("{}", x == *y);
    println!("{}", *y == *ref_1);
    // println!("{}", y == ref_1);
}

fn greetings(name: &str){
    println!("Welcome! {}", name);
}

fn deref_string(){
    greetings("Xiyuan Yang");
    greetings(&String::from("Xiyuan Yang"));
    let m = Box::new(String::from("Rust"));
    greetings(&m);
    // &m: &Box<String>
    // for deref trait, &Box<String> -> &String is available!
}


fn deref_string_2(){
    let s = String::from("Hello world");
    // Dereference s to get a &str, then get the type name of that reference.
    let x = &*s;
    println!("{}", print_type(&x));
}

