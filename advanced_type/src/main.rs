#![allow(unused)]

use std::io::Error;
use std::fmt;

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

// ========================================= //

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

// ========================================= //

    let f: Thunk = Box::new(|| println!("hi"));
}

// ========================================= //

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    f
}

// ========================================= //

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_short_type(f: Thunk) {
    // --snip--
}

fn returns_short_type() -> Thunk {
    let f: Thunk = Box::new(|| println!("hi"));
    f
}

// ========================================== //

// unuse alias
// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    fn flush(&mut self) -> std::io::Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> std::io::Result<()>;
}

// ========================================== //

fn bar() -> ! {
    loop {
        // --snip--
    }
}

