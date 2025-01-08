/*
A string is actually a vector of bytes with some extra guarantees.
 */

/// Creates a new string using String::new()
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#creating-a-new-string
pub fn create_a_new_string() -> String
{
    let mut s: String = String::new();
    s
}

/// Creates a new string using the to_string() method
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#creating-a-new-string
pub fn create_a_new_string_using_to_string() -> String
{
    // The to_string method is available on any type that 
    // implements the Display trait
    let s: String = "initial contents".to_string();
    s
}

/// Creates a new string using the String::from() function
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#creating-a-new-string
pub fn create_a_new_string_using_from() -> String
{
    // String::from and to_string do the same thing
    // They are equivalent
    let s: String = String::from("initial contents");
    s
}

/// Shows off that as long as a string is UTF-8 encoded
// it can exist in Rust
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#updating-a-string
pub fn utf8_example()
{
    // All of the below strings are valid string values in Rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

/// Shows how to update a string using the push_str method
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#updating-a-string
pub fn updating_a_string_using_push_str()
{
    let mut s: String = String::from("foo");
    // push_str appends a string slice to a String
    // this prevents push_str from taking ownership of "bar"
    s.push_str("bar");
    println!("{s}"); // prints foobar

    let s2 = "bar";
    // we can use s2 after calling push_str
    // because push_str doesn't take ownership of s2
    s.push_str(s2);
    println!("s2 is {s2}");
}

/// Shows how to concatenate strings with the + operator
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
pub fn concatenating_strings_with_plus_operator()
{
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // The + operator uses the add method
    // to concatenate the strings
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{s3}");
    
    /*
    The + operator uses the add method, which has the following signature:
    fn add(self, s: &str) -> String;
     */
}

/// Shows how to concatenate strings with the format! macro
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
pub fn concatenating_strings_with_format_macro()
{
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // The format! macro returns a String
    // and doesn't take ownership of any of its parameters
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

/// Shows how to index into a string
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#slicing-strings
pub fn indexing_into_a_string()
{
    let s = String::from("hello");
    // The [] operator is not valid for strings in Rust
    // let h = s[0]; // This will not compile
    // Instead, we can use the get method
    let h = s.get(0..1);
    println!("{:?}", h);
}

/// Shows how to iterate over a string
/// https://rust-book.cs.brown.edu/ch08-02-strings.html#methods-for-iterating-over-strings
pub fn iterating_over_strings()
{
    let s = String::from("hello");
    // The chars method returns an iterator over the characters in a string
    for c in s.chars() {
        println!("The character is: {c}");
    }
    // The bytes method returns an iterator over the bytes in a string
    for b in s.bytes() {
        println!("The byte value of the character is {b}");
    }
}