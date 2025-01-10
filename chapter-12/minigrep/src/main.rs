use std::{env, fs, process};
use minigrep::Config;

fn main() {
    /*
        std::env::args will panic if any argument contains invalid Unicode
        If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead
        That function returns an iterator that produces OsString values instead of String values
     */
    // .collect() is a method that takes the iterator and collects the resulting values into a collection data type
    let args: Vec<String> = env::args().collect();
    
    // dbg! is a macro that prints the value of an expression for debugging purposes
    /*
        The first value in the vector is "target/debug/minigrep", which is the name of our binary. 
        This matches the behavior of the arguments list in C.
        This lets programs use the name by which they were invoked in their execution
     */
    dbg!(&args);

    // https://rust-book.cs.brown.edu/ch12-01-accepting-command-line-arguments.html#saving-the-argument-values-in-variables
    // https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html#calling-configbuild-and-handling-errors
    /*
        .unwrap_or_else()
        - is a method that is defined on Result<T, E>.
        - It takes two closures as arguments.
        - If the Result is an Ok value, unwrap_or_else returns the inner value.
        - If the Result is an Err value, unwrap_or_else calls the closure that takes the error value as an argument.
        - The closure passed to unwrap_or_else in this example calls process::exit(1), which exits the program with a status code of 1.
        - The status code of 1 indicates to the operating system that the program ended with an error.
        - This allows us to implement custom non-panic! error handling
     */
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html#handling-errors-returned-from-run-in-main
    // use if let rather than unwrap_or_else to check whether run returns an Err value and to call process::exit(1) if it does
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}