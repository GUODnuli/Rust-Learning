#![allow(unused)]
fn main() {
    let mut s = String::from("hello, world!");

    let len = first_word(&s);

    println!("The first word is: {}", len);

    println!("s is {} and first word is {}", s, len);

    let s1 = String::from("hello, world!");

    let hello = &s1[..6];
    let world = &s1[6..];

    println!("{}{}", hello, world);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }

    &s[..]
}
