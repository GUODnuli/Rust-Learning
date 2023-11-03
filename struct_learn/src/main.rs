#![allow(unused)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("s2@example.com"), String::from("s2"));

    let user3 = User {
        email: String::from("s3@example.com"),
        username: String::from("s3"),
        ..user1
    };
    println!("user1 is {:?}", user1);
    println!("user2 is {:?}", user2);
    println!("user3 is {:?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
