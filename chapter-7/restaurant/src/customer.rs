﻿// The use key-word imports the hosting module into the current scope.
// This allows us to call the add_to_waitlist function without needing to use the crate::front_of_house::hosting:: prefix.
use crate::back_of_house;
use crate::front_of_house::hosting;

// Part of our crate's Public API, so we need to make it public.
pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    
    println!("order1: {:?}", &order1);
    println!("order2: {:?}", &order2);
}
