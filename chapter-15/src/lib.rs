//! # Chapter 15: Smart Pointers
//! [Documentation](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
//! 
//! # Summary
//! ## Pointers
//! - A general concept for a variable that stores a memory address.
//! - This address refers to, or “points at,” another value stored in memory.
//! - The most common kind of pointer in Rust is a reference.
//! - References are indicated by the `&` symbol and borrow the value they point to.
//! - References are immutable by default and don't have any special capabilities or overhead
//! 
//! ## Smart Pointers
//! - data structures that act like pointers but also have additional metadata and capabilities.
//! - smart pointers exist in other languages like C++
//! - Rust has a variety of smart pointers defined in the standard library.
//! - The most common smart pointers in Rust are `Box<T>`, `Rc<T>`, and `Arc<T>`.
//! - While references only borrow data, smart pointers _own_ data.
//! - Smart pointers implement the `Deref` and `Drop` traits.
//! ## Deref Trait
//! - Allows an instance of a smart pointer to be treated like a reference.
//! - This allows you to write code that operates on references and use that code with smart pointers too.
//! ## Drop Trait
//! - Allows you to customize the code that is run when an instance of a smart pointer goes out of scope.
//! - This is useful for freeing resources like file handles or network connections.
//! ## Box<T>
//! - Smart Pointer
//! - Used for allocating values on the heap.
//! - The value is stored on the heap, and the pointer to the value is stored on the stack.
//! - The value is dropped when the Box goes out of scope.
//! ## Rc<T>
//! - Reference Counted Smart Pointer
//! - Allows multiple ownership to the same object
//! - Keeps track of the number of references to the object and only drops the object when the last reference is dropped.
//! - Useful for data structures that have multiple owners.
//! ## Arc<T>
//! - Atomic Reference Counted Smart Pointer
//! - Similar to Rc<T> but is safe to use in concurrent code.
//! - The `A` stands for atomic.
//! ## Interior Mutability
//! - A design pattern in Rust that allows you to mutate data even when there are immutable references to that data.
//! - This pattern uses smart pointers to achieve this.
//! - An immutable type exposes an API for mutating the interior value

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
    }
}
