//! # Art
//!
//! A library for modeling artistic concepts.
//!
//! Based on the [Rust Brown Book - Chapter 14.2: Exporting a Convenient Public API with pub use](https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use)

/// The kinds module provides types of RYB colors according to the RYB color model.
pub mod kinds {
    #[derive(Debug, PartialEq, Clone, Copy)]
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(Debug, PartialEq)]
    /// Colors created by mixing two primary colors in equal amounts according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

/// The utilities module provides useful functions for working with RYB colors.
pub mod utils {
    use crate::kinds::*;

    /// Combines two [`PrimaryColor`]s in equal amounts to create a [`SecondaryColor`].
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        match c1 {
            PrimaryColor::Red => {
                match c2 {
                    PrimaryColor::Yellow => Some(SecondaryColor::Orange),
                    PrimaryColor::Blue => Some(SecondaryColor::Purple),
                    _ => None,
                }
            }
            PrimaryColor::Yellow => {
                match c2 {
                    PrimaryColor::Red => Some(SecondaryColor::Orange),
                    PrimaryColor::Blue => Some(SecondaryColor::Green),
                    _ => None,
                }
            }
            PrimaryColor::Blue => {
                match c2 {
                    PrimaryColor::Red => Some(SecondaryColor::Purple),
                    PrimaryColor::Yellow => Some(SecondaryColor::Green),
                    _ => None,
                }
            }
        }
    }
}
