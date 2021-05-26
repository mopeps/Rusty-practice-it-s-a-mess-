use traits_media::{Summary,Tweet};

struct Point<T> {
    x: T,
    y: T,
}

//in Method definitions we would have

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//we could alse declare
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}// which means that only Points of f32 have this method

//as with the result type,we can use generics in enums this way
enum Resulta<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 5.0, y: 4.0 }; // { x: 5, y: 4.0 }; won't compile since it has a type mismatch
/* if we define de struct as Point<T, U> {
 *                              x: T,
 *                              y: U,
 *                          } then it wouldn't have any problem but
 *  that's not the case here
 * */
    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());



    let number_list = vec![34, 50, 25, 100, 65];

    /*let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }*/

    println!("The largest number is {}", largest(&number_list));
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

 /*   let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    } */

    println!("The largest number is {}", largest(&number_list));
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

//OR WE COULD USE

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) ->  char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
/*---------A generic approach would be ------------*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
/*
 * Using various generics with traits can be written in two ways
 * fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 
 * which is okay, but the nicer way would be
 *
 * fn some_function<T, U>(t: &T, u: &U) -> i32
 *      where T: Display + Clone,
 *            U: Clone + Debug
 * {...
 *
 

//We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait, as shown here:
*/

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// However, you can only use impl Trait if you’re returning a single type.
// For example, this code that returns either a NewsArticle or 
// a Tweet with the return type specified as impl Summary wouldn’t work:
/*


fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
*/


