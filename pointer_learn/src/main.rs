// 15.1 使用Box<T>指向堆上的数据
// use pointer_learn::List::{Cons, Nil};
use pointer_learn::MyBox;

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
