fn main() {
    panic::calling_panic();
}

/// https://doc.rust-lang.org/std/macro.panic.html
/// https://rust-book.cs.brown.edu/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic
mod panic {
    /// This function will panic and print "crash and burn"
    pub fn calling_panic() {
        panic!("crash and burn");
    }
}

/// https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result
mod recoverable_errors_with_result
{
    use std::fs::File;
    use std::io::ErrorKind;

    pub fn try_to_open_file()
    {
        use std::fs::File;
        // The return type of File::open is a Result<T, E>
        let f = File::open("hello.txt");
        // Ok and Err are variants of the Result enum
        // They are both brought into scope by the prelude
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("There was a problem opening the file: {:?}", error)
            },
        };
    }
    
    /// https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors
    pub fn try_to_open_file_handle_different_errors_with_match()
    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            },
        };
    }
    
    /// https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html#alternatives-to-using-match-with-resultt-e
    pub fn try_to_open_file_handle_different_errors_with_unwrap_or_else()
    {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {error:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });
    }
    
    /// will return the value inside the Ok variant if the Result is an Ok
    /// will call panic! if the Result is and Err
    pub fn unwrap()
    {
        let f = File::open("hello.txt").unwrap();
    }
    
    /// will return the value inside the Ok variant if the Result is an Ok
    /// will call panic! with the message provided if the Result is and Err
    /// allows you to provide an error message to panic! that is more informative
    pub fn expect()
    {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
    
    ///https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html#propagating-errors
    /// 
    /// Reads a username from a file
    /// If the file doesn't exist or can't be read
    /// This function will return those errors to the code that called the function
    pub fn propagating_errors()
    {
        use std::fs::File;
        use std::io::{self, Read};

        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        }
    }
    
    /// https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    pub fn question_mark_operator()
    {
        use std::fs::File;
        use std::io;
        use std::io::Read;

        fn read_username_from_file() -> Result<String, io::Error> {
            // defined to work in almost the same way as the match expressions we defined to handle the Result values in propagating_errors
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
    }

    use std::fs;
    use std::io;
    /// Opens the file hello.txt
    /// Creates a new string
    /// Reads the contents of the file into the string
    /// Returns the string
    pub fn read_text_from_file_easy_way() -> Result<String, io::Error>
    {
        fs::read_to_string("hello.txt")
    }
}

/// https://rust-book.cs.brown.edu/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
mod guessing_game
{
    /// Represents a guess in the guessing game
    pub struct Guess {
        value: i32,
    }
    
    impl Guess {
        /// Creates a new Guess
        /// 
        /// ## Arguments
        /// 
        /// * `value` - The value of the guess
        /// 
        /// ## Returns
        /// 
        /// * A new Guess
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}");
            }
            
            Guess { value }
        }
        
        /// Returns the value of the guess
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
