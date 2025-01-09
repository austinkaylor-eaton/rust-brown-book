/*
All about Traits In Rust

Traits are a way to define shared behavior in an abstract way.
They are similar to interfaces in other programming languages.
Traits are used to define shared behavior between different types.
They are a way to define a set of methods that a type must implement in order to use the trait.

Traits are very similar to interfaces in other programming languages.
However, there are some key differences between traits and interfaces in Rust:
- Traits can have default implementations for methods.
- Traits can have associated types.
- Traits can be implemented for types outside of the current crate.

Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
 */

/// An example of how to use traits in a Rust library
/// # Remarks
/// - The rust library is a media aggregator that can aggregate media from different sources
/// - The media aggregator can display summaries of data that might be stored in a NewsArticle or Tweet
/// - A trait can have multiple methods, but only one method is required to be implemented
mod media_aggregator {
    /// A trait that defines a summary method
    /// # Remarks
    /// - This trait's summary method will be used by any media data structures that require a summary, such as Tweets or NewsArticles
    pub trait Summary {
        /// A method that returns a summary of the data structure
        /// # Returns
        /// `String` - A summary of the data structure
        /// # Explanation
        /// - This method is an example of a default implementation for a trait method
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    /// A struct that represents a news article
    /// # Remarks
    /// - This struct is used to store data about a news article
    /// - The NewsArticle struct implements the [Summary] trait
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    /// An implementation block for the NewsArticle struct
    /// # Remarks
    /// - This implementation block is used to implement the [Summary] trait for the NewsArticle struct
    /// - The NewsArticle struct must implement the [Summary] trait in order to use the summarize method
    impl Summary for NewsArticle {
        /// A method that returns a summary of the news article
        /// # Returns
        /// `String` - A summary of the news article
        /// # Explanation
        /// - This method returns a summary of the news article by combining the `headline`, `author`, and `location`
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    /// A struct that represents a tweet
    /// # Remarks
    /// - This struct is used to store data about a tweet
    /// - The Tweet struct implements the [Summary] trait
    pub struct Tweet {
        /// The username of the tweet author
        pub username: String,
        /// The content of the tweet
        pub content: String,
        /// Whether the tweet is a reply
        pub reply: bool,
        /// Whether the tweet is a retweet
        pub retweet: bool,
    }

    /// An implementation block for the Tweet struct
    /// # Remarks
    /// - This implementation block is used to implement the [Summary] trait for the Tweet struct
    /// - The Tweet struct must implement the [Summary] trait in order to use the summarize method
    impl Summary for Tweet {
        /// A method that returns a summary of the tweet
        /// # Returns
        /// `String` - A summary of the tweet
        /// # Explanation
        /// - This method returns a summary of the tweet by combining the `username` and `content`
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}

/// An example of how to use traits as parameters in Rust
/// # See Also
/// - [Brown.edu Rust Book](https://rust-book.cs.brown.edu/ch10-02-traits.html#traits-as-parameters)
mod traits_as_parameters {
    use super::media_aggregator::Summary;

    /// A function that takes a reference to a type that implements the [Summary] trait
    /// # Arguments
    /// * `item` - A reference to a type that implements the [Summary] trait
    /// # impl Trait
    /// - This is syntax sugar for a longer form that is called a `trait bound`
    /// - The `impl Trait` syntax is convenient and makes the code easier to read and write
    /// - The `impl Trait` expands to a longer form that is equivalent to the following:
    /// ```rust
    /// fn notify<T: Summary>(item: &T) {
    ///    println!("Breaking news! {}", item.summarize());
    /// }
    /// ```
    /// # Explanation
    /// - Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name
    /// - In the body of notify, we can call any methods on item that come from the Summary trait, such as `summarize`
    /// - We can call notify and pass in any instance of [crate::traits::media_aggregator::NewsArticle] or [crate::traits::media_aggregator::Tweet] because they both implement the [Summary] trait
    /// - Code that calls the function with any other type, such as a `String` or an `i32`, won’t compile because those types don’t implement Summary.
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
}

/// An example of how to use multiple `trait bounds` with the `+` syntax in Rust
/// # See Also
/// - [Brown.edu Rust Book](https://rust-book.cs.brown.edu/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax)
mod multiple_trait_bounds_with_plus_syntax {
    use super::media_aggregator::Summary;

    /// A function that takes a reference to a type that implements the [Summary] trait and the [std::fmt::Display] trait
    /// # Arguments
    /// * `item` - A reference to a type that implements the [Summary] trait and the [std::fmt::Display] trait
    /// # Explanation
    /// - This function takes a reference to a type that implements both the [Summary] trait and the [std::fmt::Display] trait
    /// - The `+` syntax is used to specify multiple trait bounds
    /// - The `+` syntax is used to specify that the item parameter must implement both the [Summary] trait and the [std::fmt::Display] trait
    pub fn notify(item: &(impl Summary + std::fmt::Display)) {
        println!("Breaking news! {}", item.summarize());
    }
}

/// An example of how to use `where` clauses to specify trait bounds in Rust
/// # See Also
/// - [Brown.edu Rust Book](https://rust-book.cs.brown.edu/ch10-02-traits.html#clearer-trait-bounds-with-where-clauses)
mod clearer_trait_bounds_with_where_clauses
{
    use std::fmt::{Debug, Display};

    /// Shows how `trait bounds` would look without the `where` clause
    fn without_where<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32;
    
    /// Shows how `trait bounds` would look with the `where` clause
    fn with_where<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug
    {0}
}

/*
You can only use impl trait if you're returning a single type from a function.

For example, you can't use impl trait if you're returning a tuple of types from a function.
 */

/// # Summary
/// - This module demonstrates how to use a `trait bound` with an `impl` block to implement methods conditionally for types that implement said trait.
mod using_trait_bounds_to_conditionally_implement_methods
{
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    /// An implementation block for the Pair struct
    /// # Explanation
    /// - the type `Pair<T>` always implements the new function to return a new instance of `Pair<T>`
    /// - recall that `Self` is a type alias for the type of the `impl` block, which in this case is `Pair<T>`
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    /* for this `impl` block, `Pair<T>` only implements the `cmp_display` 
    method if its inner type `T` implements:
     - the `PartialOrd` trait that enables comparison 
     - the `Display` trait that enables printing.
     */
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

/// # Summary
/// This module demonstrates how to use `blanket implementations` to implement a trait for all types that implement another trait.
mod blanket_implementations
{
    use std::fmt::Display;
    
    /// A trait that defines a `to_string` method
    pub trait MyToString {
        /// A method that returns a string representation of the type
        fn to_string(&self) -> String;
    }

    /*
    This is a blanket implementation of the `MyToString` trait for all types that implement the `Display` trait.
     */
    impl<T: Display> MyToString for T {
        fn to_string(&self) -> String {
            todo!()
        }
        // --snip--
    }
}
