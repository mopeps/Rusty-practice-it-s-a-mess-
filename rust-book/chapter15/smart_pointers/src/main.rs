mod boxes;
mod deref_trait;
use deref_trait::{MyBox};
//use boxes::List::{Cons, Nil};
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    println!("Hello, world!");
  //  boxes::first_example();
   // let list = Cons(1,
    //                Box::new(Cons(2,
    //                              Box::new(Cons(3,
     //                                           Box::new(Nil)
     //                                           )
      //                                     )
       //                           )
         //                    )
          //          );
    deref_trait::deref_first();

    let m = MyBox::new(String::from("Rust"));
    deref_trait::hello(&m);
    
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");    
    let aa = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&aa));
    let bb = Cons(3, Rc::clone(&aa));
    println!("count after creating b = {}", Rc::strong_count(&aa));
    {
        let cc = Cons(4, Rc::clone(&aa));
        println!("count after creating c = {}", Rc::strong_count(&aa));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&aa));
}



struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
