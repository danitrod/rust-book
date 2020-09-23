#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// front_of_house doesn't need to be set as public here because it is a sibling of eat_at
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // Use of super
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// fn _main() {
//     mod back_of_house {
//         pub struct Breakfast {
//             pub toast: String,
//             seasonal_fruit: String,
//         }

//         impl Breakfast {
//             pub fn summer(toast: &str) -> Breakfast {
//                 Breakfast {
//                     toast: String::from(toast),
//                     seasonal_fruit: String::from("peaches"),
//                 }
//             }
//         }

//         pub enum Appetizer {
//             Soup,
//             Salad,
//         }
//     }

//     pub fn eat_at_restaurant() {
//         // Order a breakfast in the summer with Rye toast
//         let mut meal = back_of_house::Breakfast::summer("Rye");
//         // Change our mind about what bread we'd like
//         meal.toast = String::from("Wheat");
//         println!("I'd like {} toast please", meal.toast);

//         // The next line won't compile if we uncomment it; we're not allowed
//         // to see or modify the seasonal fruit that comes with the meal
//         // meal.seasonal_fruit = String::from("blueberries");

//         let _order1 = back_of_house::Appetizer::Soup;
//         let _order2 = back_of_house::Appetizer::Salad;
//     }
// }


// Alias name importing
// #![allow(unused)]
// fn main() {
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
//     Ok(())
// }

// fn function2() -> IoResult<()> {
//     // --snip--
//     Ok(())
// }
// }

// Single line multiple imports
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// Import everything
// use std::collections::*;

// Using other files
mod front_of_house; // Imports ./front_of_house.rs

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
