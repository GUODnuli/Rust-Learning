#[warn(unused_imports)]
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut num_vec = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..9 {
        let n: u8 = rng.gen();
        num_vec.push(n);
    }

    for i in &num_vec {
        println!("{}", i);
    }
}
