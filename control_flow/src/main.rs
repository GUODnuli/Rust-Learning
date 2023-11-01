#![allow(unused)]
use std::io;

fn if_fn() {
    let condition: bool = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn loop_fn() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn while_fn() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LEFT OFF!!!");
}

fn for_fn() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("This value is while print:{}", a[index]);

        index += 1;
    }

    for value in a {
        println!("This value is for print:{}", value);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn f2c(value: f64, model: u8) -> f64 {
    if model == 0 {
        value * 9.0 / 5.0 + 32.0
    } else {
        (value - 32.0) * 5.0 / 9.0
    }
}

fn fibonacci(number: i8) -> String {
    let mut return_string = String::new();
    if number <= 1 {
        return_string.push_str(&number.to_string());
        return return_string
    }

    let mut previous = 0;
    let mut current = 1;
    for i in 2..number {
        return_string = format!("{}{}{}", return_string, " ",current.to_string());
        
        let next = previous + current;
        previous = current;
        current = next;
    }
    return_string
}

fn main() {
    let temperature: f64 = 25.0;
    let temperature_f: f64 = 20.0;
    let number: i8 = 15;

    println!("{}°C is {}°F", temperature, f2c(temperature, 0));
    println!("{}°F is {:.3}°C", temperature_f, f2c(temperature_f, 1));
    println!("The {}th Fibonacci number is:\n{}", number, fibonacci(number));
}
