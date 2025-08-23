#[derive(Clone, Debug)]
struct User {
    active: bool,
    user_name: String,
    id: i32,
    email: String,
}

struct NewUser<'a>{
    name: &'a str,
    email: &'a str
}

#[derive(Clone, Copy)]
struct Color(i32, i32, i32);
#[derive(Clone, Copy)]
struct Point(i32, i32, i32);

fn main() {
    println!("Hello world!");
    create_struct();

    let user_test: User = create_user(String::from("hello@sjtu.edu.cn"), String::from("wow"));
    println!("{}", user_test.active);

    let user_2 = create_user_from_existing(user_test.clone());
    println!("{}", user_2.email);
    println!("{}", user_test.active);

        let color_test = Color(0, 0, 0);
    let point_test = Point(23, 23, 23);

    let color_values = [color_test.0, color_test.1, color_test.2];
    println!("Color:");
    for value in color_values {
        println!("{}", value);
    }

    let point_values = [point_test.0, point_test.1, point_test.2];
    println!("Point:");
    for value in point_values {
        println!("{}", value);
    }

    let user = NewUser{
        name: &"hello world"[0..=2],
        email: "wow it is great"
    };

    println!("{}", user.name);
    println!("{}", user.email);
    println!("{user_test:#?}");

}

fn create_struct() {
    // using the key-value pair
    let user_1 = User {
        active: true,
        user_name: String::from("Xiyuan Yang"),
        id: 123456,
        email: String::from("test@gmail.com"),
    };

    let user_2 = User {
        active: false,
        user_name: String::from("test"),
        id: 1233445,
        email: String::from("test"),
    };

    println!("{}{}{}", user_1.active, user_1.id, user_1.email);
    println!("{}", user_2.user_name);
}

// or you can simplify this...

fn create_user(email: String, user_name: String) -> User {
    User {
        active: true,
        user_name,
        id: 12345,
        email,
    }
}

fn create_user_from_existing(user_1: User) -> User {
    let user_more = User {
        active: false,
        ..user_1
    };
    user_more
}


