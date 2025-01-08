/*
    Vectors are similar to arrays but they can grow and shrink in size. 
    They are useful when you have a collection of items that you want to store in a single variable.
    All values in a vector are put next to each other in memory
    Example use cases for a vector: 
    - lines of text in a file
    - prices of items in a shopping cart
 */
use std::slice::Iter;

/// Creates a new vector of type i32
pub fn create_new_vector() -> Vec<i32> {
    let v: Vec<i32> = Vec::new();
    v
}

/// Creates a new vector of type i32 using the vec! macro
pub fn create_vector_macro() -> Vec<i32> {
    // The vec! macro creates a new vector and initializes it with the values provided
    let v = vec![1, 2, 3];
    v
}

/// Updates a vector by adding elements to it
pub fn update_vector() -> Vec<i32> {
    let mut v = Vec::new();
    // The push method is used to add elements to a vector
    v.push(5);
    v.push(6);
    v.push(7);
    v
}

/*
There are 2 ways to get data from a vector:
- indexing
- get method

both are shown below
 */
/// Reading elements of a vector
pub fn read_vector_element() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

/// An example of trying to get a value from a vector that is outside the range of the vector
/// Expected output: thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
#[cfg(panic = "unwind")]
#[expect(index_out_of_bounds)]
pub fn index_value_outside_range() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

#[expect(E0502)]
/// An example of trying an immutable borrow with a vector
pub fn immutable_borrow() {
    /*
    because vectors put the values next to each other in memory, 
    adding a new element onto the end of the vector might require allocating new memory 
    and copying the old elements to the new space, 
    if there isn’t enough room to put all the elements next to each other where the vector is currently stored. 
    In that case, the reference to the first element would be pointing to deallocated memory
     */
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //v.push(6);

    println!("The first element is: {first}");
}

pub fn iterating_over_values() {
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        /*
        To read the number that n_ref refers to
        we have to use the * dereference operator 
        to get to the value in n_ref before we can add 1 to it
         */
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }
}

/*
to use iterators safely, Rust does not allow you to add or remove elements from the vector during iteration
 */
/// Shows a deconstruction of an iterator being used with a vector
pub fn deconstructing_iterator() {
    let mut v: Vec<i32> = vec![1, 2]; // L1
    let mut iter: Iter<'_, i32> = v.iter(); // L1
    let n1: &i32 = iter.next().unwrap(); // L2
    let n2: &i32 = iter.next().unwrap(); // L3
    let end: Option<&i32> = iter.next(); // L4
    
    println!("{:?}, {:?}, {:?}", n1, n2, end);
    
    // L1: v and iter are allocated on the stack with pointers to the heap containing 1,2
    // L2: n1 is allocated on the stack with a reference to the first element in the vector (heap)
    // L3: n2 is allocated on the stack with a reference to the second element in the vector (heap)
    // L3: iter is now empty, so the pointer to the heap is removed from the stack
    // L4: end is allocated on the stack with a value of None
}

/// Represents a cell in a spreadsheet
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

/// Shows how using an enum with a vector can be useful for storing different types of data
/// Because, remember: vectors can only store values of the same type
/// https://rust-book.cs.brown.edu/ch08-01-vectors.html#using-an-enum-to-store-multiple-types
pub fn use_enum_with_vector(){
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {value}"),
            SpreadsheetCell::Text(value) => println!("Text: {value}"),
            SpreadsheetCell::Float(value) => println!("Float: {value}"),
        }
    }
}