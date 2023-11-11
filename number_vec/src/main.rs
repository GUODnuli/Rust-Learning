#![warn(unused_imports)]
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut num_vec = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..9 {
        let n: u8 = rng.gen();
        num_vec.push(n);
    }
    
    println!("vector is {:?}", &num_vec);
    println!("avg is {}", compute_avg(&num_vec));
    println!("middle-number is {}", (&num_vec[4] + &num_vec[5]) / 2);
}

fn compute_avg(num_vec: &[u8]) -> u32 {
    let sum: u32 = num_vec.iter().map(|&val| u32::from(val)).sum();
    let avg: u32 = sum / num_vec.len() as u32;
    avg
}
