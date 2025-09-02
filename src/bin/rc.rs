use std::rc::Rc;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum List_Safe {
    Cons_Safe(i32, Rc<List_Safe>),
    Nil_Safe,
}
use crate::List::{Cons, Nil};
use crate::List_Safe::{Cons_Safe, Nil_Safe};
fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    // error: value used here after move

    // correct code
    let a = Rc::new(Cons_Safe(5, Rc::new(Cons_Safe(10, Rc::new(Nil_Safe)))));
    println!("Count: {}", Rc::strong_count(&a)); // 1
    {
        let b = Cons_Safe(3, Rc::clone(&a));
        println!("Count: {}", Rc::strong_count(&a)); // 2
        let c = Cons_Safe(4, Rc::clone(&a));
        println!("Count: {}", Rc::strong_count(&a)); // 3
    }
    println!("Count: {}", Rc::strong_count(&a)); // 1
}
