#![allow(unused)]
fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("s2");
    println!("{}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    let r1 = &mut s1;
    // 解开下面两行的注释会导致报错
    // let r2 = &mut s1;

    // println!("{}, {}", r1, r2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_string: i32) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
