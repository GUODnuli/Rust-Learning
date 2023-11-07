pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

pub mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}

    pub mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from("toast"),
                    seasonal_fruit: String::from("mongo"),
                }
            }
        }
        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }

        fn cook_order() {}
    }
}
