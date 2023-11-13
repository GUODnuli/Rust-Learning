use std::io;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("You can input like \"Add Sam to Engineering\" to add starff.");
    println!("Or you can input <apartment-name> to get ");
    println!("And you can input \"quit\" to exit this program.");
    // let mut starff_map: HashMap<,> = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();
        
        let re_input = Regex::new(r"Add ([a-zA-Z]*) to ([a-zA-Z]*)").expect("Syntax error.");
        match re_input.captures(input) {
            Some(caps) => {
                let name = caps.get(1);
                let apartment = caps.get(2);
                println!("{:?}  {:?}", name, apartment);
                // starff_map.entry(name).or_insert(apartment);
            },
            _ => {},
        }
        // println!("{:?}", starff_map);
    }
}
