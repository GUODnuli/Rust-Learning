#![allow(unused)]
mod front_of_house {
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
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 使用use导入serving模块
    use front_of_house::serving;
    // 在夏天点一份黑麦面包作为早餐
    let mut meal = serving::back_of_house::Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // 尝试更改随餐搭配的季节水果会导致编译失败
    // meal.seasonal_fruit = String::from("blueberries");
}
