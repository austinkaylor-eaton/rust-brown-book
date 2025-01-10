﻿use std::error::Error;
use std::{env, fs};

/// A function to run the program
/// # Arguments
/// * `config` - A [Config] instance with the query and file path values
/// # Returns
/// * <b>Success:</b> The contents of the file
/// * <b>Error:</b> A type that implements the [Error] trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? returns the error value from the current function for the caller to handle
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    // https://rust-book.cs.brown.edu/ch12-04-testing-the-librarys-functionality.html#using-the-search-function-in-the-run-function
    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// A struct to hold the configuration values passed in from the command line
pub struct Config {
    /// The query to search for
    pub query: String,
    /// The file path to search
    pub file_path: String,
    /// Whether to ignore case when searching
    pub ignore_case: bool,
}

impl Config {
    /// Create a new [Config] instance from a string slice
    /// # Arguments
    /// * `args` - A slice of the command line arguments
    /// # Returns
    /// * <b>Success:</b> A [Config] instance with the query and file path values
    /// * <b>Error:</b> An error message if the slice is too short
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Err is a generic enum type that represents an error
            // The &'static str type is a string slice that has the 'static lifetime
            // Error messages are usually string slices with static lifetimes 
            //      because they’re baked into the program and always available
            return Err("Not enough arguments");
        }
        /*
           We used to return string slices that reference String values in args. 
           We now define Config to contain owned String values. 
           The args variable in main is the owner of the argument values. 
           The args variable is only letting the parse_config function borrow them. 
           That means we’d violate Rust’s borrowing rules if Config tried to take ownership of the values in args
            
            Using clone() makes a full copy of the data for the Config instance to own
            The trade-off for the extra runtime cost is that we get a clear separation of concerns between main and parse_config
            If we didn't use clone(), we'd have to manage lifetimes of the borrowed references between main and parse_config
            */
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Get the value of the IGNORE_CASE environment variable
        // We’re using the is_ok method on the Result to check whether the environment variable is set
        //  If the IGNORE_CASE environment variable isn’t set to anything, is_ok will return false and the program will perform a case-sensitive search
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query, // using shorthand initialization. really reads query: query
            file_path, // using shorthand initialization. really reads file_path: file_path
            ignore_case // using shorthand initialization. really reads ignore_case: ignore_case
        })
    }
}

/// A function to search for a query in a string
/// # Arguments
/// * `query` - The query to search for
/// * `contents` - The string to search
/// # Returns
/// * A vector of string slices that match the query
/// # Remarks
/// * The function uses an explicit lifetime `'a` to ensure that the returned vector contains string slices that reference the contents string
/// * Remember, lifetime parameters specify which argument's lifetime should be used for the return value's lifetime
/// * In this case, the return value's lifetime is the same as the `contents` string
/// * So, the data returned by this function will live as long as the `contents` string
/// # Algorithm
/// 1. Iterate through each line of the contents.
/// 2. Check whether the line contains our query string.
/// 3. If it does, add it to the list of values we’re returning.
/// 4. If it doesn’t, do nothing.
/// 5. Return the list of results that match.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) { 
            results.push(line);
        }
    }
    
    results
}

/// A function to search for a query in a string in a case-insensitive manner
/// # Arguments
/// * `query` - The query to search for
/// * `contents` - The string to search
/// # Returns
/// * A vector of string slices that match the query
/// # Remarks
/// * The function uses an explicit lifetime `'a` to ensure that the returned vector contains string slices that reference the contents string
/// * Remember, lifetime parameters specify which argument's lifetime should be used for the return value's lifetime
/// * In this case, the return value's lifetime is the same as the `contents` string
/// * So, the data returned by this function will live as long as the `contents` string
/// # Algorithm
/// 1. Convert the query and line to lowercase
/// 2. Iterate through each line of the contents.
/// 3. Convert the line to lowercase
/// 4. Check whether the line contains our query string.
/// 5. If it does, add it to the list of values we’re returning.
/// 6. If it doesn’t, do nothing.
/// 7. Return the list of results that match.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}