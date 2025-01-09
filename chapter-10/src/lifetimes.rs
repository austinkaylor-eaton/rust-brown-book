/*
The main aim of lifetimes it to prevent dangling references in Rust.
A dangling reference is a reference to a value that has been deallocated from memory.
Lifetimes are a way to ensure that references in Rust are always valid.

Lifetime Annotations
- they don't change how long any of the references live
- they describe the relationships of the lifetimes of multiple references to the compiler
 */
use std::fmt::Display;

pub(crate) mod lifetime_annotations {
    fn simple_example<'a>() {
        let a: &i32 = &4; // a reference without a lifetime parameter
        let b: &'a i32 = &4; // a reference with a lifetime parameter 'a
        let mut c: &'a i32 = &4; // a mutable reference with a lifetime parameter 'a
    }

    /// This function returns the longest string between two strings.
    /// # Arguments
    /// * `x` - A reference to a string slice
    /// * `y` - A reference to a string slice
    /// # Returns
    /// `&str` - A reference to the longest string
    /// # Explanation
    /// - The function signature uses the lifetime annotation `'a` to specify that the references `x` and `y` have the same lifetime.
    /// - Furthermore, the function signature specifies that the return value has the same lifetime `'a` as parameters `x` and `y`.
    /// - This ties the lifetimes of the references `x`, `y`, and the return value together.
    /// - This means that the return value will live as long as the references `x` and `y` are in scope.
    /// - When specifying lifetime parameters in this function signature, the lifetimes of values passed into or returned from the function are not changed
    /// - Rather, the lifetime annotation `'a` is telling the borrow checker that it should reject any values that don't adhere to the specified lifetimes.
    /// # Lifetime `a
    /// - the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`.
    /// - In other words, the generic lifetime `'a` <b>will get the concrete lifetime that is equal to the smaller of the lifetimes</b> of `x` and `y`.
    /// - Because we’ve annotated the returned reference with the same lifetime parameter `'a`, the <b>returned reference will also be valid for the length of the smaller of the lifetimes</b> of `x` and `y`
    fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    /// This function demonstrates what happens when you try to call the `longest_string` function with different concrete lifetimes.
    /// # Explanation
    /// - The function creates two strings, `string1` and `string2`, with different concrete lifetimes.
    /// - `string1` has a lifetime that extends to the end of the outer scope.
    /// - `string2` has a lifetime that extends to the end of the inner scope.
    pub(crate) fn calling_longest_string_with_different_concrete_lifetimes() {
        let string1 = String::from("string one");

        {
            let string2 = String::from("string2");
            // result references something that is valid until the end of the inner scope (string2's scope)
            let result = longest_string(string1.as_str(), string2.as_str());
            // output should be "The longest string is string one"
            // result is valid until the end of string2's scope
            // so the println! macro will not be able to access the result variable if outside of string2's scope
            println!("The longest string is {result}");
            // end of string2's scope
        }
        // for example, the following code won't compile:
        // println!("The longest string is {result}");
        // end of string1's scope
    }

    /// This function demonstrates how rust handles lifetimes when a function has multiple parameters, but only 1 parameter has a lifetime
    /// # Explanation
    /// - Since the function always returns parameter `x`, the lifetime of the return value is the same as the lifetime of `x`.
    /// - The lifetime of `y` is not considered when determining the lifetime of the return value.
    /// - Therefore, `y` does not need a lifetime annotation.
    /// # See Also
    /// - [Brown.edu Rust Book - Chapter 10](https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html#thinking-in-terms-of-lifetimes)
    fn only_one_parameter_has_lifetime<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    /// A struct that will be used with the `use_important_excerpt` function.
    /// # See Also
    /// [Brown.edu Rust Book - Chapter 10](https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions)
    pub struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    /// This function demonstrates how to use lifetime annotations in struct definitions.
    /// # Explanation
    /// - The `ImportantExcerpt` struct has a lifetime annotation `'a` on the `part` field.
    /// - This annotation means an instance of `ImportantExcerpt` can't outlive the reference it holds in its `part` field.
    /// - The main function here creates an instance of the `ImportantExcerpt` struct that holds a reference to the first sentence of the String owned by the variable `novel`
    /// - The data in novel exists before the ImportantExcerpt instance is created.
    /// - In addition, novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.
    fn use_important_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}

/*
Lifetime Elision Rules
- The compiler uses lifetime elision rules to determine the lifetimes of references in function signatures.
- The rules are:
    1. (only applies to input lifetimes) Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.
    2. (only applies to output lifetimes) If there is exactly one input lifetime, elided or not, the lifetime of the return value is the same as the input lifetime.
    3. (only applies to output lifetimes) If there are multiple input lifetimes, but one of them is `&self` or `&mut self`, the lifetime of the return value is the same as the lifetime of the `self` parameter.
 
 The elision rules apply to `fn` signatures and `impl` blocks.
 */
mod lifetime_elision_rules {
    /*
    Lifetimes on function or method parameters are called `input lifetimes`
    Lifetimes on return values are called `output lifetimes`
     */
    
    /// Examples of the first lifetime elision rule
    mod first_rule_examples {
        /// This function would get 1 lifetime parameter if the elision rule was not applied.
        /// 
        /// The compiler transforms the function to:
        /// ```rust
        /// fn foo<'a>(x: &'a i32)
        /// ```
        fn a(x: &i32) {}

        /// This function would get 2 lifetime parameters if the elision rule was not applied.
        /// 
        /// The compiler transforms the function to:
        /// ```rust
        /// fn bar<'a, 'b>(x: &'a i32, y: &'b i32)
        /// ```
        fn b(x: &i32, y: &i32) {}

        use crate::lifetimes::lifetime_annotations::ImportantExcerpt;
        /// This function would get 2 lifetime parameters if the elision rule was not applied.
        /// 
        /// The compiler transforms the function to:
        /// ```rust
        /// fn c<'a, 'b>(x: &'a ImportantExcerpt<'b>)
        /// ```
        fn c(x: &ImportantExcerpt) {}
    }
    
    mod second_rule_examples {
        /// The second rule is that, if there is exactly one input lifetime parameter, 
        /// that lifetime is assigned to all output lifetime parameters 
        /// # Explanation
        /// - The function signature has one input lifetime parameter, `'a`.
        /// - The return value has the same lifetime as the input lifetime parameter.
        /// - The compiler transforms the function to:
        /// ```rust
        /// fn foo<'a>(x: &'a i32) -> &'a i32
        /// ```
        fn foo(x: &i32) -> &i32 {
            x
        }
    }
    
    mod third_rule_examples {
        struct Foo {
            x: i32,
        }
        
        /// The third rule is that, if there are multiple input lifetime parameters, 
        /// but one of them is `&self` or `&mut self`, the lifetime of the return value is the same as the lifetime of the `self` parameter.
        impl Foo {
            /// The `&self` parameter has a lifetime of `'a`.
            /// Since there are 2 input lifetime parameters, both `self` and `x` would have different lifetimes
            /// However, one of them is `&self`, so the return value has the same lifetime as the `&self` parameter.
            /// The return value has the same lifetime as the `&self` parameter.
            /// The compiler transforms the function to:
            /// ```rust
            /// fn bar<'a, 'b>(&'a self, x: &'b i32) -> &'a i32 {
            ///     x
            /// }
            /// ```
            fn bar(&self, x: &i32) -> &i32 {
                x
            }
        }
    }
}

/// This function shows how to use `generic type parameters`, `trait bounds`, and `lifetimes` in a single function.
/// # Arguments
/// * `x` - A reference to a string slice
/// * `y` - A reference to a string slice
/// * `ann` - A generic type parameter that implements the `Display` trait
/// # Returns
/// `&str` - A reference to the longest string
/// # Explanation
/// - The extra parameter `ann` is a generic type parameter that implements the `Display` trait.
/// - The `Display` trait is used to print the value of `ann` in the `println!` macro.
/// - Because lifetimes are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle brackets after the function name
fn all_in_one<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
