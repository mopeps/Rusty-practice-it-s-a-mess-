pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn first_example() {
    let b = Box::new(5);

    println!("b = {}", b);
}

