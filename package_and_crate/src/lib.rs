#![allow(unused)]
// 声明front_of_house模块
mod front_of_house;
// 使用use导入serving模块
use front_of_house::serving;

pub fn eat_at_restaurant() {
    // 在夏天点一份黑麦面包作为早餐
    let mut meal = serving::back_of_house::Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // 尝试更改随餐搭配的季节水果会导致编译失败
    // meal.seasonal_fruit = String::from("blueberries");
}
