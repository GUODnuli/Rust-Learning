use std::io;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("You can input like \"Add Sam to Engineering\" to add starff.");
    println!("Or you can input \"get <apartment-name>\" to get all starff name in <apartment-name>");
    println!("And you can input \"quit\" to exit this program.");
    let mut starff_map: HashMap<String, String> = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();
        if input == "quit" {
            break;
        }

        let operate = Regex::new(r"([\D]+)")
            .expect("Failed to initialize regex rule.");
        match operate.captures(input) {
            Some(caps) => {
                let operate_math = caps.get(1).unwrap().as_str().to_string();
                println!("{}", operate_math);
            },
            None => {},
        }
        
        let re_input = Regex::new(r"Add ([a-zA-Z]*) to ([a-zA-Z]*)")
            .expect("Failed to initialize regex rule.");        
        match re_input.captures(input) {
            Some(caps) => {
                let name = caps.get(1).unwrap().as_str().to_string();
                let apartment = caps.get(2).unwrap().as_str().to_string();

                println!("{}: {}", apartment, name);
                starff_map.entry(name).or_insert(apartment);
            },
            None => println!("Syntax error.\nPlease input like \"Add <name> to <apartment>\"."),
        }
        println!("{:?}", starff_map);
    }
}
