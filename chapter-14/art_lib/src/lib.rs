//! # Art
//!
//! A library for modeling artistic concepts.
//! 
//! Based on the [Rust Brown Book - Chapter 14.2: Exporting a Convenient Public API with pub use](https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use)

/// The kinds module provides types of RYB colors according to the RYB color model.
pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

/// The utilities module provides useful functions for working with RYB colors.
pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
