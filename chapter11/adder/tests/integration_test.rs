use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup;
    assert_eq!(4, adder::add_two(2));
}



//the code below was used in the first part of this chapter as unit tests
/*    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    } 
 //   #[test]
 //   fn it_adds_two() {
  //      assert_eq!(4, add_two(2));
  //  }

    #[test]
    fn a_failure() {
        assert_eq!(2 + 2, 5);
    }

//checking results with the assert! macro
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width : 8,
            height : 7,
        };
        let smaller = Rectangle {
            width : 5,
            height : 1,
        };
        assert!(larger.can_hold(&smaller));
    } 
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
//Adding Custom Failure Messages 
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );    
    }
    //Checking for panics and should panics

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
//Using Result<T, E> in tests
     #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }   
 */   

