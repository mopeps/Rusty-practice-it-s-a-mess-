fn main() {

    interactingWithData2();
    example_of_borrowing();
}
/*
fn first_example() {
    //the variable s refers to a string literal
    //And its validity goes along the scope of this function
    let s = "hello";

}

fn string_type() {
    /* unlike string literals,the String type is allocated on
    the heap. Plus,this kind of string can be mutated*/
    
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string
    println!("{}",s); // will print hello world
} // the scope is now over,so we free 's'
//this is due to rust calling the function 'drop' automatically 
//when closing a curly bracket,its similar to RAII



fn interactingWithData1() {
    let x = 5;
    let y = x; // because integers are simple types,each variable
    //holds a copy of the value 5 in this case


    //with complex data types like strings this is not the case
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!",s1);

}*/

fn interactingWithData2() {

    //this is how you would do a deep copy of the heap data of the string
    //instead of invalidating s1 and moving s1 to s2.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    //now if we try this it won't crash
    println!("s1 = {} , and s2 = {}",s1,s2);
}

fn example_of_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    

    println!("The length of '{}' is {}",s1,len);


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

