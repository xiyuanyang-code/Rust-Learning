fn main() {
    println!("Hello world!");
    expression();
    println!("Demo for pow function: {}", pow(10, 2));

    if_else();

    println!("{}", always_running());

    loop_label();
    loop_label_2();
    loop_label_3();

    println!("Fibonacci: {}", fibonaci(5));
}

fn expression() {
    let x = if 1 == 2 { 5 } else { 10 };
    let y = {
        let z = 1;
        z + 2
    };
    println!("x value is {}", x);
    println!("y value is {}", y);
}

fn pow(x: i32, p: u32) -> i64 {
    let mut result: i64 = 1;
    for _i in 0..p {
        result = result * (x as i64);
    }
    result
}

fn if_else() {
    let x = 1;
    if x == 1 {
        println!("It is True");
    }
}

fn always_running() -> i32 {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    result
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Counting up end.")
}

fn loop_label_2() {
    let mut i = 0;

    'outer_while: while i < 5 {
        println!("外层循环: i = {}", i);

        let mut j = 0;
        'inner_while: while j < 5 {
            println!("  内层循环: j = {}", j);

            if j == 2 {
                // 这个 break 语句只会退出内层循环
                break 'inner_while;
            }
            if i == 3 {
                // 使用循环标签，直接跳出外层循环
                break 'outer_while;
            }
            j += 1;
        }

        i += 1;
    }

    println!("循环结束");
}

fn loop_label_3() {
    'outer_for: for i in 1..=3 {
        println!("外层循环: i = {}", i);

        'inner_for: for j in 1..=3 {
            println!("  内层循环: j = {}", j);

            if i == 2 && j == 2 {
                // 这个 `break` 语句会直接跳出外层的 'outer_for 循环
                // 整个程序将提前结束
                break 'outer_for;
            }

            if i == 3 {
                break 'inner_for;
            }
        }
    }

    println!("所有循环已结束");
}

fn fibonaci(index: i32) -> i32 {
    if index == 0 {
        return 0;
    };
    if index == 1{
        return 1;
    }

    let mut pre = 1;
    let mut pre_pre = 0;
    for _ in 2..=index {
        let next = pre + pre_pre;
        pre_pre = pre;
        pre = next;
    };

    pre
}
