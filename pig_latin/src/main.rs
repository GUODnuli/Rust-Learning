use std::io;
use regex::Regex;

fn main() {
    let re_vowel = Regex::new(r"^[aeiouAEIOU]").unwrap();

    loop {
        println!("You can input some words to change into Pig-Latin.");
        println!("And you can input \"quit\" to exit this program.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();
        if input == "quit" {
            break;
        }

        if re_vowel.is_match(input) {
            println!("The Pig-Latin is {}-hay", input);
        } else {
            let first_char = input.chars().next().unwrap();
            let rest_chars = &input[first_char.len_utf8()..];
            println!("The Pig-Latin is {}-{}ay", rest_chars, first_char);
        }
    }
}
