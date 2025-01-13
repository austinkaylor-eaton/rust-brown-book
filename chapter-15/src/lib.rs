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

/// Module 15.1 - Using Box<T> to Point to Data on the Heap
/// # See
/// - [Box<T>](https://doc.rust-lang.org/std/boxed/struct.Box.html)
/// - [Rust Book - Chapter 15.1](https://doc.rust-lang.org/book/ch15-01-box.html)
mod box_pointer {
    
}

/// Module 15.2 - Treating Smart Pointers Like Regular References with the Deref Trait
/// # See
/// - [Deref Trait](https://doc.rust-lang.org/std/ops/trait.Deref.html)
/// - [Rust Book - Chapter 15.2](https://doc.rust-lang.org/book/ch15-02-deref.html)
mod deref_trait {
    
}

/// Module 15.3 - Running Code on Cleanup with the Drop Trait
/// # See
/// - [Drop Trait](https://doc.rust-lang.org/std/ops/trait.Drop.html)
/// - [Rust Book - Chapter 15.3](https://doc.rust-lang.org/book/ch15-03-drop.html)
mod drop_trait {
    
}

/// Module 15.4 - Rc<T>, the Reference Counted Smart Pointer
/// # See
/// - [Rc<T>](https://doc.rust-lang.org/std/rc/struct.Rc.html)
/// - [Rust Book - Chapter 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html)
mod rc_pointer {
    
}

/// Module 15.5 - RefCell<T> and the Interior Mutability Pattern
/// # See
/// - [RefCell<T>](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
/// - [Rust Book - Chapter 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
mod refcell {
    
}

/// Module 15.6 - Reference Cycles Can Leak Memory
/// # See
/// - [Rust Book - Chapter 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
mod reference_cycles {
    
}

