#[derive(Debug)]
enum IpAddrkind {
    V4,
    V6,
}

#[derive(Debug, Clone)]
enum IpAddr {
    V4(String),
    V6(String),
}

// using the abstract data class
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage {
    write_info: String,
}

struct ChangeColorMessage(i32, i32, i32);

enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}

fn main() {
    println!("Hello world");
    let four = IpAddrkind::V4;
    let six = IpAddrkind::V6;
    println!("{four:#?}");
    println!("{six:#?}");

    let home = IpAddr::V4(String::from("127.0.0,1"));
    println!("{home:#?}");

    route(&IpAddrkind::V4);

    let message_test = Message::Write(WriteMessage {
        write_info: String::from("test"),
    });

    let some_number = Some("5");
    let absent_number: Option<i32> = None;
    
}

fn route(ip: &IpAddrkind) {
    println!("{ip:#?} is running!");
}
