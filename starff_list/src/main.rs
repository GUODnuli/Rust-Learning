use std::io;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("You can input like \"Add Sam to Engineering\" to add starff.");
    println!("Or you can input \"get <department-name>\" to get all starff name in <department-name>");
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

        let operate = Regex::new(r"([a-zA-Z]+)")
            .expect("Failed to initialize regex rule.");
        match operate.captures(input) {
            Some(caps) => {
                let operate_type = caps.get(1).unwrap().as_str().to_string();
                if operate_type == "Add" {
                    add_employee(input, &mut starff_map);
                } else if operate_type == "Get" {
                    get_employee(input, &mut starff_map);
                }
            },
            None => {},
        }
    }
}

fn add_employee(input: &str, starff_map: &mut HashMap<String, String>) {
    let re_input = Regex::new(r"Add ([a-zA-Z]+) to ([a-zA-Z]+)")
            .expect("Failed to initialize regex rule.");        
    match re_input.captures(input) {
        Some(caps) => {
            let name = caps.get(1).unwrap().as_str().to_string();
            let department = caps.get(2).unwrap().as_str().to_string();

            println!("{}: {}", department, name);
            starff_map.entry(name).or_insert(department);
        },
        None => println!("Syntax error.\nPlease input like \"Add <name> to <department>\"."),
    }
    println!("{:?}", starff_map);
}

fn get_employee(input: &str, starff_map: &mut HashMap<String, String>) {
    let re_input = Regex::new(r"Get ([a-zA-Z]+)")
        .expect("Failed to initialize regex rule.");
    match re_input.captures(input) {
        Some(caps) => {
            let department = caps.get(1).unwrap().as_str().to_string();

            for (key, value) in starff_map.iter().filter(|&(_, v)| v == &department) {
                println!("{} in {}", key, &department);
            }
        },
        None => println!("Syntax error.\nPlease input like \"Get <department>\"."),
    }
}