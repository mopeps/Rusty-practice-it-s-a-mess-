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
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}




