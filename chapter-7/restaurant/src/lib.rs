/*
Example of a restaurant module with nested modules.

modules can hold definitions for other items like structs, enums, constants, traits, functions, or other modules.
 */
mod front_of_house;
mod back_of_house;
mod customer;

fn deliver_order() {}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
/* 
Module Tree for the restaurant module:
crate
    └── front_of_house
        ├── hosting   
        |     ├── add_to_waitlist
        |     └── seat_at_table
        └── serving
               ├── take_order
               ├── serve_order
               └── take_payment
 */

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
