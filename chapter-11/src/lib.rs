/// Adds two numbers
/// # Arguments
/// - `left`: A u64 number
/// - `right`: A u64 number
/// # Returns
/// - A u64 number
/// - The sum of `left` and `right`
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Represents a Rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// Implementation of the Rectangle struct
impl Rectangle {
    /// Checks if a rectangle can hold another rectangle
    /// # Arguments
    /// - `other`: A reference to a Rectangle
    /// # Returns
    /// - A [bool]
    /// - `true` if the rectangle can hold the other rectangle, `false` otherwise
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Adds two to a number
/// # Arguments
/// - `a`: A usize number
/// # Returns
/// - A usize number
/// - The sum of `a` and 2
pub fn add_two(a: usize) -> usize {
    a + 2
}

/// Greets a person by name
/// # Arguments
/// - `name`: [&str] A string slice representing the name of the person
/// # Returns
/// - [String] "Hello `name`!" 
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

/// Represents a Guess
pub struct Guess {
    value: i32,
}

/// Implementation of the [Guess] struct
impl Guess {
    /// Creates a new [Guess] instance
    /// # Arguments
    /// - `value`: An i32 number
    /// # Returns
    /// - A [Guess] instance
    /// # Panics
    /// - If the value is less than 1 or greater than 100
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the [add] function
    /// # Remarks
    /// - The #test attribute tells Rust to compile and run the test code only when we run cargo test.
    /// - The #test attribute indicates that this is a test function, so the test runner knows to call this function when running the tests.
    /// - The `assert_eq!` macro is provided by the standard library, and it verifies that its two arguments are equal.
    /// - We can also have non-test functions in the tests module that are not considered tests and won’t be run unless we specifically call them.
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    /// This test should fail
    /// # Remarks
    /// - The `panic!` macro causes the test to fail.
    /// - Tests fail when something in the test code panics.
    /// - Each test is run in a new thread
    /// - When the main thread sees that a test thread has died, it reports the test as failed.
    /// - The test runner captures the output of the test thread and prints that output when the test thread panics.
    #[test]
    #[ignore] // This test is ignored
    fn another() {
        panic!("Make this test fail");
    }

    /// Tests the [can_hold] method of the Rectangle struct to see if a larger rectangle can hold a smaller rectangle
    /// # Expected Result
    /// - `true` because the larger rectangle should be able to hold the smaller rectangle
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    /// Test the [can_hold] method of the Rectangle struct to see confirm that a smaller rectangle cannot hold a larger rectangle
    /// # Expected Result
    /// - `false` because the smaller rectangle should not be able to hold the larger rectangle
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert_eq!(smaller.can_hold(&larger), false);
    }

    /// Test the [add_two] function to confirm that it adds two to a number
    /// # Expected Result
    /// - `4` because 2 + 2 = 4
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    
    /*
        Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively
        When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits
        All primitive types and most of the standard library types implement these traits
        
        For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types. 
        You’ll also need to implement Debug to print the values when the assertion fails
     
        Because both traits are derivable traits, this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition.
     */

    /// Test the [greeting] function to confirm that it greets a person by name
    /// # Expected Result
    /// - "Hello Carol!" because the function should return a greeting with the name passed as an argument
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    /// Test the [Guess] struct's [Guess::new()] method to confirm that it panics when the value is greater than 100
    /// # Expected Result
    /// - The test should panic
    /*
        The #[should_panic] annotation on the test function tells the test runner that this test should panic.
        If the code inside the test function does not panic, the test will fail.
        
        Tests that use #[should_panic] can be imprecise alone.
        A should_panic test would pass even if the test panics for a different reason from the one we were expecting
     
        Adding an optional expected parameter to the #[should_panic] annotation can make the test more precise.
        For example, the expected parameter in the annotation for the greater_than_100 test function is expected to be "Guess value must be between 1 and 100, got 200."
     */
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")] 
    fn greater_than_100() {
        Guess::new(200);
    }

    /// Test the [add] function with a Result return type
    /// # Expected Result
    /// - `Ok(())` because the function should return `Ok(())` when the sum of the two numbers is 4
    #[test]
    /*
        Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, 
        which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
     
        You can’t use the #[should_panic] annotation on tests that use Result<T, E>
        To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value
        Instead, use assert!(value.is_err()) or assert_eq!(value, Err(…))
     */
    fn it_works_with_result() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
