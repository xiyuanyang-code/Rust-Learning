use std::mem::size_of;

fn main() {
    let x = 100;
    println!("Current x value: {x}");

    // available
    let mut x = 10;
    println!("x is now changed into {x}");
    x = 100;
    println!("x is now changed into {x}");

    // x = 100;
    println!("x is now changed into {x}");

    // for constants
    inner_function();

    variables();

    compound();
}

fn inner_function() {
    const CONST_VARIABLE: i32 = 60 * 60 * 3;
    println!("We define the const variable: {CONST_VARIABLE}");
}

fn variables() {
    println!("--- 标量类型 ---");
    println!("i8:    {} bytes", size_of::<i8>());
    println!("u8:    {} bytes", size_of::<u8>());
    println!("i16:   {} bytes", size_of::<i16>());
    println!("u16:   {} bytes", size_of::<u16>());
    println!("i32:   {} bytes", size_of::<i32>());
    println!("u32:   {} bytes", size_of::<u32>());
    println!("i64:   {} bytes", size_of::<i64>());
    println!("u64:   {} bytes", size_of::<u64>());
    println!("i128:  {} bytes", size_of::<i128>());
    println!("u128:  {} bytes", size_of::<u128>());
    println!("isize: {} bytes", size_of::<isize>());
    println!("usize: {} bytes", size_of::<usize>());
    println!("f32:   {} bytes", size_of::<f32>());
    println!("f64:   {} bytes", size_of::<f64>());
    println!("bool:  {} bytes", size_of::<bool>());
    println!("char:  {} bytes", size_of::<char>());

    println!("\n--- 复合类型 ---");
    println!("():      {} bytes", size_of::<()>()); // 空元组 (Unit)
    println!("(i32, f64): {} bytes", size_of::<(i32, f64)>()); // 元组
    println!("[i32; 3]: {} bytes", size_of::<[i32; 3]>()); // 数组

    // 一个简单的结构体
    struct MyStruct {
        x: i32,
        y: bool,
    }

    let my_instance = MyStruct { x: 10, y: true };

    println!("{}", my_instance.x);
    println!("{}", my_instance.y);
    println!("MyStruct: {} bytes", size_of::<MyStruct>());
}

fn compound() {
    let a = [3; 5];
    println!("{}", a[0]);
}
