struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        sign_in_count: 1,
        active: true,
    };

    let user3 = User {
        email: String::from("dummie@example.com"),
        username: String::from("robert69420"),
        ..user1
    };
    let black = Color(0,0,0);

    let mut user2 = build_user(String::from("arandom@example.com"),String::from("djkhaled"));
    user1.email = String::from("anotherOne@example.com");
    println!("{}",user1.email);
}

