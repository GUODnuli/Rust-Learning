#![warn(unused_imports)]
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut num_vec = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let n: u8 = rng.gen();
        num_vec.push(n);
    }
    
    println!("vector is {:?}", &num_vec);
    println!("avg is {}", compute_avg(&num_vec));
    num_vec.sort();
    println!("The sorted vector is {:?}", &num_vec);
    let num1: u32 = num_vec[4] as u32;
    let num2: u32 = num_vec[5] as u32;
    println!("middle-number is {}", (num1 + num2) / 2);

    let mut map = HashMap::new();
    for num in &num_vec {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("The number count is {:?}", map)
}

fn compute_avg(num_vec: &[u8]) -> u32 {
    let sum: u32 = num_vec.iter().map(|&val| u32::from(val)).sum();
    let avg: u32 = sum / num_vec.len() as u32;
    avg
}
