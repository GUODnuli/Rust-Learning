#![allow(unused)]

fn add_one(arg: i32) -> i32 {
    arg + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let result = do_twice(add_one, 5);

    println!("Result: {}", result);
}
