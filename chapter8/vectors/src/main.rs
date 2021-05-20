fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2]; //---> return i32

    println!("The third element is {}",third);

    match v.get(2) { // ---> return Option<i32>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
}   
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

