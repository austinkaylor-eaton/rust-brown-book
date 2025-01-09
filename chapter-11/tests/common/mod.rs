/*
    Modules don't work the same way in integration tests as they do in the rest of the codebase.
    
    In order to have "shared" code that can be used in multiple integration tests, we can create a `common` module.
    However, we need to follow the old Rust naming convention and create a `common` directory with a `mod.rs` file.
    
    The `mod.rs` file is used to define the contents of the `common` module.
 */

pub fn setup() {
    // setup code specific to your library's tests would go here
}