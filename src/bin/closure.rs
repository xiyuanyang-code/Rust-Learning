use std::thread;

fn main() {
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");
        let only_borrows = || println!("From closure: {list:?}");
        println!("Before calling closure: {list:?}");
        only_borrows();
        println!("After calling closure: {list:?}");
    }

    {
        let mut mut_list = vec![1, 2, 3];
        println!("Before defining closure: {mut_list:?}");
        let mut mut_borrows = || {
            mut_list.push(23);
            println!("After modification in the closure: {mut_list:?}");
        };
        // it will create a mutable reference
        // println!("Before calling closure: {mut_list:?}");
        // not allowed here
        mut_borrows();
        println!("After calling closure: {mut_list:?}");
    }

    {
        let list = vec![1, 3, 4];
        println!("Before defining: {list:?}");
        thread::spawn(move || println!("From thread: {list:?}"))
            .join()
            .unwrap()
    }
}
