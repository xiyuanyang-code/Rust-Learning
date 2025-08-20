enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Write(String::from("Hello"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Quit;
    let msg4 = Message::ChangeColor(255, 0, 100);

    match msg1 {
        Message::Write(text) => {
            println!("Received a message: {}", text);
        }
        _ => {}
    }

    match msg2 {
        Message::Move { x, y } => {
            println!("Received a move command to x:{} and y:{}", x, y);
        }
        _ => {}
    }

    match msg3 {
        Message::Quit => {
            println!("Received a quit command.");
        }
        _ => {}
    }

    match msg4 {
        Message::ChangeColor(r, g, b) => {
            println!("Received a change color command to R:{} G:{} B:{}", r, g, b);
        }
        _ => {}
    }
}
