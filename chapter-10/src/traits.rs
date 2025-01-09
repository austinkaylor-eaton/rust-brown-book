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
mod media_aggregator
{
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