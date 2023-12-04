use std::ops::Deref;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T);

pub impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}