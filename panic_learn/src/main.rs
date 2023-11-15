#![warn(unused_imports)]

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    // Will panic!
    // let v = vec![1, 2, 3];
    // v[99];

    // Result与可恢复的错误
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating th file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // 进阶写法
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // 失败时panic的简写：unwrap和expect
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // 传播错误
    // let s = String::from("hello.txt");
    // read_username_from_file(&s);

    let f = File::open("hello.txt")?;

    Ok(())
}

// fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    // let f = File::open(path);

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // ?运算符
    // let mut f = File::open(path)?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // ?运算符与链式方法
    // let mut s = String::new();

    // File::open(path)?.read_to_string(&mut s)?;
    // Ok(s)

//     fs::read_to_string(path)
// }