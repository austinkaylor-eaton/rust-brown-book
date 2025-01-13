#![deny(missing_docs)] // Tells Rust to treat undocumented code as an error
//! # Art
//!
//! A library for modeling artistic concepts.
//!
//! Based on the [Rust Brown Book - Chapter 14.2: Exporting a Convenient Public API with pub use](https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use)

// Re-export the kinds and utils modules.
// Makes as better API for the users of the art library.
// https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

/// The kinds module provides types of RYB colors according to the RYB color model.
pub mod kinds {
    #[derive(Debug, PartialEq, Clone, Copy)]
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        /// The color red.
        Red,
        /// The color yellow.
        Yellow,
        /// The color blue.
        Blue,
    }

    #[derive(Debug, PartialEq)]
    /// Colors created by mixing two primary colors in equal amounts according to the RYB color model.
    pub enum SecondaryColor {
        /// The color orange.
        Orange,
        /// The color green.
        Green,
        /// The color purple.
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

#[cfg(test)]
mod tests {
    use crate::kinds::PrimaryColor;
    use crate::utils::mix;

    /// Tests the [`mix`] function with the [`PrimaryColor::Red`] and [`PrimaryColor::Yellow`] colors.
    #[test]
    fn red_and_yellow_make_orange() {
        let red = PrimaryColor::Red;
        let yellow = PrimaryColor::Yellow;
        let result = mix(red, yellow);
        assert_eq!(result, Some(crate::kinds::SecondaryColor::Orange));
    }

    /// Tests the [`mix`] function with the [`PrimaryColor::Red`] and [`PrimaryColor::Blue`] colors.
    #[test]
    fn red_and_blue_make_purple() {
        let red = PrimaryColor::Red;
        let blue = PrimaryColor::Blue;
        let result = mix(red, blue);
        assert_eq!(result, Some(crate::kinds::SecondaryColor::Purple));
    }

    /// Tests the [`mix`] function with the [`PrimaryColor::Yellow`] and [`PrimaryColor::Blue`] colors.
    #[test]
    fn yellow_and_blue_make_green() {
        let yellow = PrimaryColor::Yellow;
        let blue = PrimaryColor::Blue;
        let result = mix(yellow, blue);
        assert_eq!(result, Some(crate::kinds::SecondaryColor::Green));
    }

    /// Tests the [`mix`] function with the [`PrimaryColor::Red`] and [`PrimaryColor::Red`] colors.
    #[test]
    fn red_and_red_make_none() {
        let red = PrimaryColor::Red;
        let result = mix(red, red);
        assert_eq!(result, None);
    }

    /// Tests the [`mix`] function with the [`PrimaryColor::Yellow`] and [`PrimaryColor::Yellow`] colors.
    #[test]
    fn yellow_and_yellow_make_none() {
        let yellow = PrimaryColor::Yellow;
        let result = mix(yellow, yellow);
        assert_eq!(result, None);
    }

    /// Tests the [`mix`] function with the [`PrimaryColor::Blue`] and [`PrimaryColor::Blue`] colors.
    #[test]
    fn blue_and_blue_make_none() {
        let blue = PrimaryColor::Blue;
        let result = mix(blue, blue);
        assert_eq!(result, None);
    }
}