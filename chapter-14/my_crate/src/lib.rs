//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds two numbers and returns the result.
/// # Arguments
/// * `left`: [u64] - The first number to add
/// * `right`: [u64] - The second number to add
/// # Returns
/// * [u64] - The sum of the two numbers
/// # Examples
/// ## Simple addition
/// This example shows how to use the `add` function to add two numbers.
/// ```rust
/// let result = my_crate::add(2, 2);
/// assert_eq!(result, 4);
/// ```
/// # See
/// - [Rust Brown Book - Chapter 14.2: Making Useful Documentation Comments](https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test case for the [`add`] function that adds two `even` numbers.
    #[test]
    fn add_even_numbers() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    /// Test case for the [`add`] function that adds two `odd` numbers.
    #[test]
    fn add_odd_numbers() {
        let result = add(3, 2);
        assert_eq!(result, 5);
    }
}
