use std::collections::HashMap;

/// Creating a new hash map using HashMap::new() and .insert
/// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#creating-a-new-hash-map
pub fn create_a_new_hash_map() -> HashMap<String, i32>
{
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores
}

/// Accessing values in a HashMap using .get
/// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#accessing-values-in-a-hash-map
pub fn accessing_values_in_a_hash_map()
{
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name: String = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    match score {
        Some(value) => println!("{value}"),
        None => println!("No value found"),
    }
}

/// Iterating over a HashMap
/// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#iterating-over-a-hash-map
pub fn iterating_over_a_hash_map()
{
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

/// Hash Maps and Ownership rules
/// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#hash-maps-and-ownership
pub fn hash_maps_and_ownership()
{
    let field_name: String = String::from("Favorite color");
    let field_value: String = String::from("Blue");

    let mut map: HashMap<&String, &String> = HashMap::new();
    map.insert(&field_name, &field_value);

    // field_name and field_value are now invalid
    // because they were moved into the map
    // println!("{field_name}, {field_value}");
}

/// Overwriting a value in a HashMap
/// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#overwriting-a-value
pub fn overwriting_a_value_in_a_hash_map()
{
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // This code will print {"Blue": 25}. The original value of 10 has been overwritten.
    println!("{scores:?}");
}

/// Adding a key and value to a hash map only if the key is not already present
/// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#adding-a-key-and-value-only-if-a-key-isnt-present
pub fn adding_key_if_not_present()
{
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    /*
    Hash maps have a special API for this called entry that takes the key you want to check as a parameter
    The return value of the entry method is an enum called Entry that represents a value that might or might not exist
     */
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

/// Updating a value based on the old value
/// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value
pub fn updating_value_based_on_old_value()
{
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // split_whitespace() returns an iterator over sub-slices, separated by whitespace
    // or_insert method returns a mutable reference (&mut V) to the value for the specified key
    for word in text.split_whitespace() {
        // &mut V is stored in count
        let count = map.entry(word).or_insert(0);
        // to use count, we have to dereference it
        *count += 1;
    }

    // This code will print {"world": 2, "hello": 1, "wonderful": 1}
    // The key-value pairs might be in a different order because hash maps do not guarantee order
    // Iterating over a hash map will always return key-value pairs in arbitrary order
    println!("{map:?}");
}