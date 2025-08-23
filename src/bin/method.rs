#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

impl Rectangle {
    fn square(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    println!("Hello world!");
    let reca = Rectangle {
        width: 100,
        height: 100,
    };
    println!("{reca:#?}");
    println!("Area: {}", reca.area());

    let mut reca_2 = Rectangle {
        width: 100,
        height: 100,
    };

    reca_2.set_width(300);
    println!("{}", reca_2.area());


    let reca_3 = Rectangle::square(100, 100);
    println!("{}", reca_3.area());
}
