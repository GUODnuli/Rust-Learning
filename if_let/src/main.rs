#![allow(unused)]
fn main() {
    match_fn();
    if_let_fn();
}

fn match_fn() {
    let some_u8_value = Some(8u8);
    match some_u8_value {
        Some(3) => println!("Three."),
        Some(8) => println!("Eight."),
        _ => (),
    }
}

fn if_let_fn() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("Tree.");
    }
}
