// 定义 PlayerAction 事件的子类型
enum ActionType {
    Attack,
    UseItem,
}

// 定义游戏事件
enum Event {
    Quit,
    Move { x: i32, y: i32, speed: f32 },
    PlayerAction(u32, ActionType),
    NetworkData(String, Vec<u8>),
}

fn handle_event(event: Event) {
    match event {
        // 匹配并解构 Move 事件，将 x, y, speed 绑定到新变量
        Event::Move { x, y, speed } => {
            println!("玩家移动到 ({}, {})，速度：{}", x, y, speed);
        }

        // 匹配并解构 PlayerAction 事件，使用 (player_id, action) 模式
        // 在此处，我们进一步使用 if 守卫来匹配特定的动作类型
        Event::PlayerAction(player_id, action) => {
            // if let 是 match 的简化情况，适用于二元关系的判断（语法糖）
            if let ActionType::Attack = action {
                println!("玩家 {} 发动了攻击！", player_id);
            } else {
                println!("玩家 {} 执行了其他动作。", player_id);
            }
        }

        // 匹配并解构 NetworkData，同时使用 if 守卫来检查字符串
        Event::NetworkData(ref protocol_name, ref data) if protocol_name == "PING" => {
            println!("收到 PING 协议数据，长度：{}", data.len());
        }

        // 匹配并解构 NetworkData，但不对协议名做任何检查
        Event::NetworkData(protocol_name, data) => {
            println!("收到 {} 协议数据，数据长度：{}", protocol_name, data.len());
        }

        // 匹配剩下的所有情况
        Event::Quit => {
            println!("游戏退出事件被触发。");
        }

        _ => {
            println!("Something Error Occur");
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let event1 = Event::Move {
        x: 10,
        y: 20,
        speed: 5.5,
    };
    let event2 = Event::PlayerAction(123, ActionType::Attack);
    let event3 = Event::PlayerAction(456, ActionType::UseItem);
    let event4 = Event::NetworkData("PING".to_string(), vec![1, 2, 3]);
    let event5 = Event::NetworkData("HEARTBEAT".to_string(), vec![4, 5]);

    handle_event(event1);
    handle_event(event2);
    handle_event(event3);
    handle_event(event4);
    handle_event(event5);

    let five: i32 = 6;
    let mut wrapped_five = Some(five);
    wrapped_five = plus_one(wrapped_five);
    println!("{:?}", wrapped_five);
}
