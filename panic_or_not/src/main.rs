fn main() {
    // check value everywhere
    // loop {
    //     // --snip--

    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     if guess < 1 || guess > 100 {
    //         println!("The secret number will be between 1 and 100.");
    //         continue;
    //     };

    //     match guess.cmp(&secret_number){
    //         // --snip--
    //     }
    // }

    // create a new type to check value
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess{
                value
            }
        }

        pub fn get_value(&self) -> i32 {
            self.value
        }
    }
}
