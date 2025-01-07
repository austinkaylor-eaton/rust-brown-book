/*
    Vectors are similar to arrays but they can grow and shrink in size. 
    They are useful when you have a collection of items that you want to store in a single variable.
    All values in a vector are put next to each other in memory
    Example use cases for a vector: 
    - lines of text in a file
    - prices of items in a shopping cart
 */
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

    v.push(6);

    println!("The first element is: {first}");
}