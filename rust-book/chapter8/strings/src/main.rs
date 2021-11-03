fn main() {
    println!("Hello, world!");
    let mut s = String::new();
    
    //ways to store data into a string

    let data = "initial contents";
    

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");


    let mut s = String::from("foo");

    s.push_str("bar"); // appends bar to the end of s

    let mut s1 = String::from("foo");

    let s2 = "bar";

    s1.push_str(s2);
    //push_str takes a string slice since we don't want to take
    //ownership of the parameter
    println!("s2 is {}",s2); // is going to compile without issues
    let mut s = String::from("lo");
    s.push('s'); //takes a single char and appends it

    /* Concatenation with + or the format! Macro */

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //We can
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    //or better, we can:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}
