#![allow(unused)]

fn main() {
    let mut s = String::new();
//====================================================//
    let data = "initial contents";
    let s = data.to_string();

    // to_string()也可直接用于字符串字面量
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
//====================================================//
    let hello = String::from("السلام عليكم");
    println!("{}", hello);

    let hello = String::from("Dobrý den");
    println!("{}", hello);

    let hello = String::from("Hello");
    println!("{}", hello);

    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);

    let hello = String::from("नमस्ते");
    println!("{}", hello);

    let hello = String::from("こんにちは");
    println!("{}", hello);

    let hello = String::from("안녕하세요");
    println!("{}", hello);

    let hello = String::from("你好");
    println!("{}", hello);

    let hello = String::from("Olá");
    println!("{}", hello);

    let hello = String::from("Здравствуйте");
    println!("{}", hello);

    let hello = String::from("Hola");
    println!("{}", hello);
//====================================================//
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
//====================================================//
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1: {}", s1);
    println!("s2: {}", s2);
//====================================================//
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
//====================================================//
    let s1 = String::from("Hello, ");
    let s2 = String::from("world.");
    let s3 = s1 + &s2;
//====================================================//
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
//====================================================//
    let s1 = String::from("hello");
    // let h = s1[0];
//====================================================//
    let len1 = String::from("Hola").len();
    let len2 = String::from("Здравствуйте").len();
    println!("Hola len is: {}", len1);
    println!("Здравствуйте len is: {}", len2);
//====================================================//
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
//====================================================//
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
