/*
    Each file in the `tests` directory is compiled as its own separate crate.
    The `tests` directory is an integration test directory.
    Integration tests are used to test how multiple parts of code work together.
    
    We need to add the `use` statement to bring the `add_two` function into scope.
    
    We don't need to use #[cfg(test)] because the tests directory is only compiled when we run `cargo test`.
 
    To run a particular test, use the `--test` flag with the name of the test file.
    For example, cargo test --test integration_tests only runs the tests in the integration_tests.rs file.
 
    Special note about Integration Tests for Binary Crates:
    - If our project is a binary create (src/main.rs), 
        we can't create integration tests in the tests directory,
        and we can't use `use` to bring functions into scope.
     - Only library crates expose functions that other crates can use; binary crates are meant to be run on their own
     
     - This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. 
     - Using that structure, integration tests can test the library crate with use to make the important functionality available. 
     - If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.
     
 */
mod common;

use chapter_11::add_two;

/// Integration test for the [add_two] function.
/// # Expected
/// `result` should be equal to `4`.
#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}