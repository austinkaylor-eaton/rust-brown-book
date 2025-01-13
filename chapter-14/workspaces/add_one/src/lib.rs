/// Adds one to the given number.
/// # Arguments
/// * `x`:[i32] - The number to add one to.
/// # Returns
/// * [i32] - The result of adding one to [x]
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_where_x_is_2_should_be_3() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
