use std::io::stdin;
fn main() {
    println!("Hello, what's your name?");
    println!("Hello {}", write_your_name());
}

fn write_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}
