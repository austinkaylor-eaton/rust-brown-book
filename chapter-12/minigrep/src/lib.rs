use std::error::Error;
use std::fs;

/// A function to run the program
/// # Arguments
/// * `config` - A [Config] instance with the query and file path values
/// # Returns
/// * <b>Success:</b> The contents of the file
/// * <b>Error:</b> A type that implements the [Error] trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? returns the error value from the current function for the caller to handle
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

/// A struct to hold the configuration values passed in from the command line
pub struct Config {
    /// The query to search for
    pub query: String,
    /// The file path to search
    pub file_path: String,
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

        Ok(Config {
            query, // using shorthand initialization. really reads query: query
            file_path // using shorthand initialization. really reads file_path: file_path
        })
    }
}