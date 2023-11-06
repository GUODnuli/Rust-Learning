#![allow(unused)]
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {x: u32, y: u32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("{}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to: R{}, G{}, B{}", r, g, b),
        }
    }
}
fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello, world!"));
    m.call()
}

fn route(ip_type: IpAddr) { }
