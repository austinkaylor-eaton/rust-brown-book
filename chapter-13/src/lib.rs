//! # Chapter 13 - Functional Language Features: Iterators and Closures
//! # [Documentation](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
//! 
//! ## 13.1 Closures: Anonymous Functions that Can Capture Their Environment
//! ### What are closures?
//! - Rust's closures are anyonymous functions you can save in a variable or pass as arguments to other functions.
//! - You can create the closure in one place and call the closure to evaluate the code in the closure at a later time.
//! - Unliked functions, closures capture values from the scope in which they're defined.
//! - Closures can capture values from their environment in three ways:
//!     - Taking ownership
//!     - Borrowing mutably
//!     - Borrowing immutably
//! 
//! 
//! ## 13.2 Processing a Series of Items with Iterators
//! 
//! ## 13.3 Improving Our I/O Project
//! 
//! ## 13.4 Comparing Performance: Loops vs. Iterators

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Shows how to use closures in Rust to capture values from the environment they're defined in for later use
/// # Scenario
/// - Every so often, our t-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list as a promotion. 
/// - People on the mailing list can optionally add their favorite color to their profile. 
/// - If the person chosen for a free shirt has their favorite color set, they get that color shirt. 
/// - If the person hasn’t specified a favorite color, they get whatever color the company currently has the most of
/// # See 
/// - [Brown Rust Book - 13.1: Capturing the Environment with Closures](https://rust-book.cs.brown.edu/ch13-01-closures.html#capturing-the-environment-with-closures)
mod closures_scenario {
    #[derive(Debug, PartialEq, Copy, Clone)]
    /// The shirt colors the company offers
    enum ShirtColor {
            Red,
            Blue
    }
    
    /// The inventory of shirts the company has
    struct Inventory {
        shirts: Vec<ShirtColor>
    }

    impl Inventory {
        /// Gives away a shirt to a user based on their preference
        /// # Arguments
        /// * `user_preference` - The user's favorite color
        /// # Returns
        /// * The color of the shirt the user will receive
        /// # Remarks
        /// * If the user has a favorite color, they will receive that color shirt
        /// * If the user does not have a favorite color, they will receive the most stocked color shirt
        /// ## unwrap_or_else
        /// - Takes one argument: a closure that returns a value of the same type as the `Option` being unwrapped.
        /// - If the `Option` is `Some`, the value is returned. 
        /// - If the `Option` is `None`, the closure is called and its result is returned.
        /// - We specify the closure expression `|| self.most_stocked()` as the _argument_ to `unwrap_or_else`. 
        /// - This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical bars).
        /// - The body of the closure calls `self.most_stocked()`. 
        /// - We’re defining the closure here, and the implementation of `unwrap_or_else` will evaluate the closure later if the result is needed
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        /// Determines the most stocked color of shirts
        /// # Returns
        /// * The color of the shirt that is most stocked
        /// # Remarks
        /// * If the company has more red shirts than blue shirts, the function will return `ShirtColor::Red`
        /// * If the company has more blue shirts than red shirts, the function will return `ShirtColor::Blue`
        /// * If the company has an equal number of red and blue shirts, the function will return `ShirtColor::Red`
        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        
        /// Basic test from the Rust Book
        /// # Remarks
        /// * the store defined has 2 blue shirts and 1 red shirt remaining to distribute for this limited edition giveaway
        /// * the [`Inventory::giveaway`] function is called for:
        ///     * a user with a preference for a red shirt
        ///     * a user without any color preference
        #[test]
        fn basic_test() {
            let store = Inventory {
                shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
            };

            let user_pref1 = Some(ShirtColor::Red);
            let giveaway1 = store.giveaway(user_pref1);
            assert_eq!(giveaway1, ShirtColor::Red);

            let user_pref2 = None;
            let giveaway2 = store.giveaway(user_pref2);
            assert_eq!(giveaway2, ShirtColor::Blue);
        }

        #[test]
        fn test_giveaway_with_user_preference() {
            let inventory = Inventory {
                shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
            };
            let user_preference = Some(ShirtColor::Blue);
            let result = inventory.giveaway(user_preference);
            assert_eq!(result, ShirtColor::Blue);
        }

        #[test]
        fn test_giveaway_without_user_preference() {
            let inventory = Inventory {
                shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
            };
            let user_preference = None;
            let result = inventory.giveaway(user_preference);
            assert_eq!(result, ShirtColor::Red);
        }

        #[test]
        fn test_most_stocked() {
            let inventory = Inventory {
                shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
            };
            let result = inventory.most_stocked();
            assert_eq!(result, ShirtColor::Red);
        }
    }
}

/// Shows how closures don't require you to annotate the types of the parameters or the return value like functions do
mod closure_type_inference_and_annotation {
    use std::thread;
    use std::time::Duration;

    /// Demonstrates how to define a closure that takes a `u32` and returns a `u32`
    /// Also demonstrates how to annotate the types of the parameters and the return value
    /// # Remarks
    /// - Closure is stored in a variable rather than defining the closure inline as an argument to a function
    fn closure_as_variable() {
        let expensive_closure = |num: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };
    }
}

/// Shows the 3 different ways closures can capture values from their environment
/// # Remarks
/// - Closures can capture values from their environment in three ways
/// - These ways are based on 3 ways a function can take parameters:
///     - borrowing immutable references
///     - borrowing mutable references
///     - taking ownership
/// # See
/// [Brown Rust Book - 13.1: Capturing References or Moving Ownership](https://rust-book.cs.brown.edu/ch13-01-closures.html#capturing-references-or-moving-ownership)
mod capturing_references_or_moving_ownership {
    use std::thread;

    /// define a closure that captures an immutable reference to the vector named list because it only needs an immutable reference to print the value
    /// # Remarks
    /// - This example also illustrates that a variable can bind to a closure definition
    /// - We can later call the closure by using the variable name and parentheses as if the variable name were a function name.
    /// - Because we can have multiple immutable references to `list` at the same time, `list` is still accessible from the code before the closure definition, after the closure definition but before the closure is called, and after the closure is called. 
    /// - This code compiles, runs, and prints:
    ///     - Before defining closure: [1, 2, 3]
    ///     - Before calling closure: [1, 2, 3]
    ///     - From closure: [1, 2, 3]
    ///     - After calling closure: [1, 2, 3]
   fn borrowing_immutable()
   {
       let list = vec![1, 2, 3];
       println!("Before defining closure: {list:?}");

       let only_borrows = || println!("From closure: {list:?}");

       println!("Before calling closure: {list:?}");
       only_borrows();
       println!("After calling closure: {list:?}");
   }
    
    /// define a closure that captures a mutable reference to the vector named `list` because it needs to change the vector by calling push
    fn borrowing_mutable()
    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        let mut borrows_mutably = || list.push(7);
        // Can't do this because we can't borrow `list` mutably and immutably in the same scope
        // println! will borrow `list` immutably, but the closure borrows `list` mutably
        // println!("Before calling closure: {list:?}");

        borrows_mutably();
        println!("After calling closure: {list:?}");
    }
    
    /// An example of a closure that takes ownership of the environment
    /// # Remarks
    /// - The closure takes ownership of the vector `list` and moves it into the closure when the closure is defined.
    /// - The closure is then responsible for `list`, and the original vector is no longer accessible in the main code after the closure definition.
    /// - The closure takes ownership of `list` by using the `move` keyword before the parameter list.
    fn taking_ownership()
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        thread::spawn(move || println!("From thread: {list:?}"))
            .join()
            .unwrap();

        println!("Before defining closure: {list:?}");
    }
}

/// Shows how to move values out of closures
/// # Remarks
/// - Once a closure has captured a reference (mutable or immutable) or ownership of a value from the environment where the closure is defined, the code in the body of the closure defines what happens to the references or values when the closure is evaluated later
/// - This affects anything that potentially moves out of the closure when the closure is called
/// - A closure body can do any of the following:
///     - move a captured value out of the closure
///     - mutate the captured value
///     - neither move nor mutate the value
///     - capture nothing from the environment to begin with
/// 
/// ## Fn Traits
/// - The way a closure captures and handles values from it's defined environment affects what traits the closure implements
/// - Each trait defines the kinds of closures a function or struct is allowed to use
/// - Closures will automatically implement one, two, or all three of the following `Fn` traits in an additive way, depending on how the closures body handles the value or reference.
/// ### FnOnce
/// - applies to closures that can be called once
/// - All closures implement this trait by default because all closures can be called at least once
/// - A closure that moves capture values out of it's body will implement `FnOnce` and not `Fn` or `FnMut`, because it can only be called once
/// ### FnMut
/// - applies to closures that don't move capture values of their body, but might mutate the captured values
/// - These closures can be called more than once
/// ### Fn
/// - applies to closures that don't move captured values out of their body and don't mutate captured values
/// - also applies to closures that capture nothing from their environment
/// - These closures can be called more than once without mutating the environment
/// - These closures are the most flexible and can be called more than once, especially in cases where the closure is being called multiple times concurrently
/// # See
/// [Brown Rust Book - 13.1: Moving Values Out of Closures and the Fn Traits](https://rust-book.cs.brown.edu/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits)
mod moving_capture_values_out_of_closures_and_the_fn_traits {
    /// Represents a rectangle with a width and height
    /// # Remarks
    /// - ## #[derive(Debug)]` annotation 
    /// - Will print the struct in a readable format
    /// - This is useful for debugging purposes
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        /// A function that uses an `FnOnce` closure to modify the `Rectangle`
        /// # Arguments
        /// * `self` - The `Rectangle` struct
        /// * `f` - A closure that takes ownership of the `Rectangle` and returns a modified `Rectangle`
        /// # Returns
        /// * The modified `Rectangle` struct
        /// # Example
        /// - In this example, the modify function is used to apply a closure that modifies the `width` and `height` of the `Rectangle`. 
        /// - The closure takes ownership of the `Rectangle`, modifies its fields, and returns the modified `Rectangle`
        /// ```rust
        /// let rect = super::Rectangle { width: 30, height: 50 };
        /// println!("Original rectangle: {:?}", rect);
        /// 
        /// let modified_rect = rect.modify(|mut r| {
        ///     r.width += 10;
        ///     r.height += 20;
        ///     r
        /// });
        /// 
        /// println!("Modified rectangle: {:?}", modified_rect);
        fn modify<F>(self, f: F) -> Rectangle
        where
            F: FnOnce(Rectangle) -> Rectangle,
        {
            f(self)
        }
    }

  
}

/// Demonstrates how using closures must name captured lifetimes
/// # Notes
/// - When using or designing functions that accept or return closures, you'll need to think about the lifetimes of the data that the closures capture
/// # See
/// [Brown Rust Book - 13.1: Closures Must Name Captured Lifetimes](https://rust-book.cs.brown.edu/ch13-01-closures.html#closures-must-name-captured-lifetimes)
mod closures_must_name_captured_lifetimes {
    /// Makes a cloner closure that captures a reference to a string slice
    /// # Arguments
    /// * `s_ref` - A reference to a string slice
    /// # Returns
    /// * A closure that clones the string slice
    /// # Remarks
    /// -  Example of using a closure with lifetimes to ensure the closure doesn't outlive the data it captures
    /// - The closure captures a reference to a string slice, so it needs a lifetime annotation to specify that the returned closure can't outlive the data it captures
    /// - The lifetime annotation in the `impl` trait definition specifies that the returned closure captures a reference to a string slice with the same lifetime as the reference passed in
    /// - The `+ 'a` syntax is a trait bound that specifies the returned closure captures a reference with the same lifetime as the reference passed in
    /// # Example
    /// ```rust
    /// // s_own gets Read and Ownership rights
    /// let s_own = String::from("hello");
    /// // s_own loses Ownership rights to the closure make_a_cloner
    /// // make_a_cloner gains Read and Ownership rights to s_own
    /// let cloner = super::make_a_cloner(&s_own);
    /// // Rust recognizes that as long as make_a_cloner is in use and scope, s_own can't be dropped
    /// drop(s_own);
    /// cloner();
    /// ```
    fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
        move || s_ref.to_string()
    }
}
