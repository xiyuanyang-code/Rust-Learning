use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });
    println!("{}{}", Rc::strong_count(&leaf), Rc::strong_count(&branch)); // 2,1
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("{}{}", Rc::strong_count(&leaf), Rc::strong_count(&branch)); // 2,1
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("{}{}", Rc::strong_count(&leaf), Rc::strong_count(&branch)); // 2,1
}
