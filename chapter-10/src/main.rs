mod traits;
mod lifetimes;

use lifetimes::lifetime_annotations;
fn main() {
    lifetime_annotations::calling_longest_string_with_different_concrete_lifetimes();
}

/// Finds the largest number in a list of numbers.
/// # Example
/// ```
/// let number_list = vec![34, 50, 25, 100, 65];
/// find_largest_number_in_list(&number_list);
/// // should output "The largest number is 100"
/// ```
/// # Arguments
/// * `number_list` - An immutable reference to a slice of i32 integers.
/// # Returns
/// () - This function does not return a value.
/// # Panics
/// This function will panic if the `number_list` is empty.
/// # Safety
/// This function is not marked as unsafe.
/// # References
/// [Rust Book - Chapter 10](https://rust-book.cs.brown.edu/ch10-00-generics.html#removing-duplication-by-extracting-a-function)
fn find_largest_number_in_list(number_list: &[i32]) {
    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

/// Finds the largest item in a list of items.
/// # Example
/// ```
/// let char_list = vec!['y', 'm', 'a', 'q'];
/// let result = largest(&char_list);
/// assert_eq!(result, &'y');
/// ```
/// # Arguments
/// * `list` - An immutable reference to a slice of items.
/// # Returns
/// `&T` - A reference to the largest item in the list.
/// # Panics
/// This function will panic if the `list` is empty.
/// # Explanation
/// This function uses a generic type `T` to find the largest item in a list of items.
/// <br></br>
/// This function is an example of how to use Generics in Rust
fn largest<T: PartialOrd>(list: &[T]) -> &T 
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}    

/// A generic struct that holds two values of the same type.
/// # Example
/// ```
/// let integer = Point { x: 5, y: 10 };
/// let float = Point { x: 1.0, y: 4.0 };
/// ```
/// # Explanation
/// - This struct is an example of how to use Generics in Rust.
/// - The `Point` struct is generic over some type `T`.
/// - The `Point` struct has two fields, `x` and `y`, both of which are of type `T`.
struct Point<T> {
    x: T,
    y: T,
}

/// An implementation block for the `Point` struct.
/// # Explanation
/// - This implementation block is an example of how to implement methods on a Generic struct in Rust.
/// - The `impl` block is used to define methods on the `Point` struct.
impl<T> Point<T> {
    /// A method that returns a reference to the `x` field of the `Point` struct.
    /// # Example
    /// ```
    /// let p = Point { x: 5, y: 10 };
    /// println!("p.x = {}", p.x());
    /// ```
    /// # Returns
    /// `&T` - A reference to the `x` field of the `Point` struct.
    fn x(&self) -> &T {
        &self.x
    }
}

/// And f32-specific implementation block for the `Point` struct.
/// # Explanation
/// - This implements a specific method for the `Point` struct when `T` is of type `f32`.
/// - Other instances of the `Point` struct with different types will not have this method.
/// - Only instances of the `Point` struct with `T` as `f32` will have this method.
impl Point<f32> {
    /// A method that calculates the distance of the `Point` struct from the origin.
    /// # Example
    /// ```
    /// let p = Point { x: 5.0, y: 10.0 };
    /// println!("Distance from origin: {}", p.distance_from_origin());
    /// ```
    /// # Returns
    /// `f32` - The distance of the `Point` struct from the origin.
    /// # Explanation
    /// - This method calculates the distance of the `Point` struct from the origin using the Pythagorean theorem.
    /// - The distance is calculated as the square root of the sum of the squares of the `x` and `y` fields.
    /// - The `x` and `y` fields are of type `f32`.
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// A generic struct that holds two values of different types.
/// # Example
/// ```
/// let p1 = Point2 { x: 5, y: 10.0 };
/// let p2 = Point2 { x: 1.0, y: 4 };
/// ```
/// # Explanation
/// - This struct is an example of how to use Generics in Rust.
/// - The `Point2` struct is generic over two types `T1` and `T2`.
/// - The `Point2` struct has two fields, `x` and `y`, one of type `T1` and the other of type `T2`.
/// - The `x` field is of type `T1` and the `y` field is of type `T2`.
struct Point2<T1, T2> {
    x: T1,
    y: T2,
}

/// A generic struct that holds two values of different types.
/// # Parameters
/// * `X1` - The type of the `x` field.
/// * `Y1` - The type of the `y` field.
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

/// An implementation block for the `Point3` struct.
impl<X1, Y1> Point3<X1, Y1> {
    /// A method that takes another `Point3` struct and returns a new `Point3` struct with the `x` field from `self` and the `y` field from `other`.
    /// # Parameters
    /// * `X2` - The type of the `x` field of the other `Point3` struct.
    /// * `Y2` - The type of the `y` field of the other `Point3` struct.
    /// * `self` - The `Point3` struct that the method is called on.
    /// * `other` - The other `Point3` struct that is passed as an argument.
    /// # Returns
    /// `Point3<X1, Y2>` - A new `Point3` struct with the `x` field from `self` and the `y` field from `other`.
    /// # Explanation
    /// - This example demonstrates a situation where some generic parameters are declared with `impl` and some are declared with the method definition, such as `mixup`.
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_number_in_list() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        assert_eq!(result, &100);
    }
    
    #[test]
    fn test_find_largest_char_in_list() {
        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        assert_eq!(result, &'y');
    }
}


