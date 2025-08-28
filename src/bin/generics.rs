struct Point<T> {
    x: T,
    y: T,
}

struct Point_tuple<T>(T, T);

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    fn spec_for_i32(&self) -> &i32 {
        println!("It is specially designed for i32 type!");
        &self.x
    }
}

// impl<T> Point<T> {
//     fn mixed_point<X>(&self, other: &Point<X>) -> (&T, &X) {
//         (&self.x, &other.y)
//     }
// }

impl<T> Point<T> {
    fn mixed_point<X>(self, other: Point<X>) -> (T, X) {
        (self.x, other.y)
    }
}

impl<T> Point<T> {
    fn mixed_point_life<'a, 'b, X>(&'a self, other: &'b Point<X>) -> (&'a T, &'b X) {
        (&self.x, &other.y)
    }
}

fn main() {
    println!("Hello world!");
    let test = vec![1, 23, 5, 6];
    let result = find_max_value(&test).unwrap();
    println!("{}", result);

    let empty_test: Vec<i32> = Vec::new();
    let result = find_max_value(&empty_test);
    println!("{result:?}");

    // using generics
    let numbers_vec = vec![1, 5, 2, 8, 3];
    let numbers_slice = [10, 5, 20, 15];
    let empty_vec: Vec<i32> = vec![];

    // 使用 Vec
    if let Some(max) = find_max_value_gen(&numbers_vec) {
        println!("The max value in vec is: {}", max); // Output: The max value in vec is: 8
    }

    // 使用 slice
    if let Some(max) = find_max_value_gen(&numbers_slice) {
        println!("The max value in slice is: {}", max); // Output: The max value in slice is: 20
    }

    // 处理空输入
    if let None = find_max_value_gen(&empty_vec) {
        println!("The input collection is empty."); // Output: The input collection is empty.
    }

    let struct_test_1 = Point { x: 100, y: 100 };

    let struct_test_2 = Point { x: 100.1, y: 200.9 };

    println!("{}", struct_test_1.get_x());
    println!("{}", struct_test_2.get_x());
    println!("{}", struct_test_1.spec_for_i32());
}

fn find_max_value(numbers: &Vec<i32>) -> Result<&i32, String> {
    if numbers.is_empty() {
        return Err("The input vector cannot be empty.".to_string());
    }

    let mut max_num = &numbers[0];

    for num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }

    Ok(max_num)
}

fn find_max_value_gen<'a, I, T>(numbers: I) -> Option<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    T: Ord + 'a,
{
    let mut numbers_iter = numbers.into_iter();

    if let Some(mut max_num) = numbers_iter.next() {
        for num in numbers_iter {
            if num > max_num {
                max_num = num;
            }
        }
        Some(max_num)
    } else {
        None
    }
}

fn find_max_for_vec<T: Ord>(numbers: &Vec<T>) -> Option<&T> {
    // Ord is a trait, ensuring generic T must can be compared
    if numbers.is_empty() {
        return None;
    }

    let mut max_num = &numbers[0];

    for num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }

    Some(max_num)
}

fn find_max_for_list<T: Ord>(numbers: &[T]) -> Option<&T> {
    if numbers.is_empty() {
        return None;
    }

    let mut max_num = &numbers[0];

    for num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }

    Some(max_num)
}
