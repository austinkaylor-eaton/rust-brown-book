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
/// # Notes
/// ## Box<T>
/// ### Summary
/// - Basic smart pointer
/// - Allocates values on the heap instead of the stack
/// - What remains on the stack is a pointer to heap data
/// - Don't have a performance overhead
/// - Don't have any extra capabilities either
/// ### Usage
/// - When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
/// - When you have a large amount of data and you want to transfer ownership but ensure data won't be copied when you do
/// - When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type
/// ## Recursive Types
/// - A type that refers to itself in some way
/// - Can have another value of the same type as part of itself
/// - Pose an issue because Rust needs to know how much space a type takes up at compile time
/// ## Cons List
/// - Data type common in functional programming languages
/// - A data structure that comes from Lisp
/// - Made up of nested pairs
/// - Lisp version of a Linked List
/// - Name cames from the `cons` function that constructs lists in Lisp
/// - By calling `cons` with an element and a list, you can create a new list with the element at the front and the rest of the list following
/// - Each item in a `cons` list contains 2 elements:
///     - The value of the current item
///     - A pointer to the next item
/// The last item contains a special value `Nil` that indicates the end of the list
/// ## Nil
/// - A special value that indicates the end of a `cons` list
/// - Canonical name to denote the base case of recursion
/// - This is not the same as `null` or `nil` in other languages
/// ## When to use [Box<T>], [Rc<T>], and [RefCell<T>]
/// Here is a recap of the reasons to choose [Box<T>], [Rc<T>], or [RefCell<T>]:
/// - [Rc<T>] enables multiple owners of the same data; [Box<T>] and [RefCell<T>] have single owners.
/// - [Box<T>] allows immutable or mutable borrows checked at compile time; [Rc<T>] allows only immutable borrows checked at compile time; [RefCell<T>] allows immutable or mutable borrows checked at runtime.
/// - Because [RefCell<T>] allows mutable borrows checked at runtime, you can mutate the value inside the [RefCell<T>] even when the [RefCell<T>] is immutable.
mod box_pointer {
    use crate::box_pointer::List::{Cons, Nil};

    /// Basic usage of the [Box<T>] smart pointer
    /// # Explanation
    /// - Define a variable `b` to have a value of [Box] that points to the value `5`
    /// - The value `5` is stored on the heap
    /// - The variable `b` is stored on the stack
    /// - Just like any other owned value, when a box goes out of scope, its value is dropped
    /// - So, the value `5` is dropped when `b` goes out of scope
    fn basic_usage() {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    /// Recursive data structure representing a Lisp Cons List in Rust
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    /// Cons List example
    /// # Explanation
    /// - Define a variable `list` that contains a [`Cons`] variant
    /// - The first [`Cons`] value holds 1 and another List value. 
    /// - This List value is another [`Cons`] value that holds 2 and another List value. 
    /// - This List value is one more [`Cons`] value that holds 3 and a List value, which is finally Nil, the non-recursive variant that signals the end of the list.
    fn cons_list() {
        let list = Cons(
            1,
            Box::new(
                Cons(
                    2,
                    Box::new(
                        Cons(
                            3,
                            Box::new(Nil),
                        )
                    ),
                )
            ),
        );
    }
}

/// Module 15.2 - Treating Smart Pointers Like Regular References with the Deref Trait
/// # See
/// - [Deref Trait](https://doc.rust-lang.org/std/ops/trait.Deref.html)
/// - [Rust Book - Chapter 15.2](https://doc.rust-lang.org/book/ch15-02-deref.html)
/// # Notes
/// ## Deref Trait
/// - Allows you to customize the behavior of the dereference operator: `*`
/// - Allows you to treat a smart pointer like a regular reference 
mod deref_trait {
    use std::ops::Deref;

    /// Basic usage of the Deref Trait with a regular reference
    /// # Explanation
    /// - Define a variable `x` with a value of `5`
    /// - Define a variable `y` that holds a reference to `x`
    /// - Dereference `y` to get the value of `x`
    /// - The dereference operator `*` allows you to get the value of a reference from the reference itself
    fn regular_pointer_dereference() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    
    /// Basic usage of the Deref Trait with a Box<T> smart pointer
    /// # Explanation
    /// - This is a rewrite of the previous example [`regular_pointer_dereference`] using a [`Box<T>`] smart pointer
    /// - The main difference between [`regular_pointer_dereference`] and this example is that `y` is a [`Box<T>`] smart pointer instead of a reference
    /// - `y` points to a copied value of `x` on the heap rather than a reference pointing to the value of `x`
    fn using_box_t_like_a_reference()
    {
        let x = 5;
        let y = Box::new(x);
        
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    /// A custom smart pointer
    struct MyBox<T>(T);

    /// Implementations for the MyBox<T> smart pointer
    impl<T> MyBox<T> {
        /// Create a new instance of the MyBox<T> smart pointer
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    /// Implement the [Deref] trait for the [`MyBox<T>`] smart pointer
    impl<T> Deref for MyBox<T> {
        type Target = T; // alias for the type of the value that the Deref trait will return

        fn deref(&self) -> &Self::Target {
            &self.0 // returns a reference to the value we want to access with `*`
        }
    }
    
    /// Example of using the [MyBox<T>] smart pointer
    fn use_my_box() {
        let x = 5;
        let y = MyBox::new(x);
        
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}

/// Module 15.3 - Running Code on Cleanup with the Drop Trait
/// # See
/// - [Drop Trait](https://doc.rust-lang.org/std/ops/trait.Drop.html)
/// - [Rust Book - Chapter 15.3](https://doc.rust-lang.org/book/ch15-03-drop.html)
mod drop_trait {
    /// Custom smart pointer that implements the Drop trait
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        /// Called when the CustomSmartPointer goes out of scope
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    /// Shows an example of using the CustomSmartPointer smart pointer
    fn drop_trait_example() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }
}

/// Module 15.4 - Rc<T>, the Reference Counted Smart Pointer
/// # See
/// - [Rc<T>](https://doc.rust-lang.org/std/rc/struct.Rc.html)
/// - [Rust Book - Chapter 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html)
/// # [Rc<T>]
/// - Reference Counted Smart Pointer
/// - Allows multiple ownership to the same object
/// - Keeps track of the number of references to the object and only drops the object when the last reference is dropped
/// - Useful for data structures that have multiple owners
/// - <b>Not</b> thread safe meaning use it when you have a single-threaded application
mod rc_pointer {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    /// Example of using the [Rc<T>] smart pointer with the [List] data structure
    /// # Explanation
    /// - Define a variable `a` that holds a reference counted smart pointer to a List value
    /// - Each cons variant will now hold a value and a [Rc<T>] pointing to a List value
    /// - When `b` is created, instead of taking ownership, it will clone the [Rc<List>] of `a`
    /// - This increases the number of references to the List value from one to two
    /// - When `c` is created, it will clone the [Rc<List>] of `a` as well
    /// - This increases the number of references to the List value from two to three
    /// - Every time a new reference is created, the reference count is incremented
    /// - When a reference goes out of scope, the reference count is decremented
    /// - When the reference count reaches zero, the List value is dropped
    /// # Rc::clone
    /// - The `clone` method only increments the reference count
    /// - Does not make a deep-copy clone of the data the `Rc<T>` points to
    /// - This is a shallow copy
    /// - This should not cause a performance overhead
    fn main() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
    }
    
    fn reference_counting() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

/// Module 15.5 - RefCell<T> and the Interior Mutability Pattern
/// # See
/// - [RefCell<T>](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
/// - [Rust Book - Chapter 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
/// # Notes
/// ## Interior Mutability
/// - A design pattern that allows you to mutate data even when there are immutable references to that data
/// - Normally, this action is disallowed by the borrowing rules
/// - This pattern uses the [unsafe] code inside a data structure to get around the borrow-checker
/// ## unsafe
/// - This indicates to the compiler that we're checking rules manually instead of relying on the compiler
/// - This is a way to tell the compiler that we're taking responsibility for enforcing the safety rules
/// ## RefCell<T>
/// - Represents single ownership over the data it holds
/// - Borrowing rules are enforced at runtime instead of compile time like [Rc<T>] and [Box<T>]
/// - If you break the rules, your program will panic and exit
/// - Useful in scenarios where the compiler can't understand the code but the code follows the safety rules
/// - Not thread safe - use for single-threaded applications only
mod refcell {
    /// Custom Trait that defines the `Messenger` interface
    pub trait Messenger {
        /// Sends a message
        /// # Arguments
        /// - `msg` - The message to send
        /// # Returns
        /// - `()` - Returns nothing
        /// # Explanation
        /// - This method takes an immutable reference to self and a string slice
        /// - This trait is the interface our mock object needs to implement so that the mock can be used in place of the real value
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::cell::RefCell;
        use super::*;

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            //assert_eq!(mock_messenger.sent_messages.len(), 1);
        }
    }
}

/// An example showing how to use [RefCell<T>] with [Rc<T>]
mod refcell_with_rc {
    /// Using [RefCell<T>] with [Rc<T>]
    /// # Explanation
    /// - This allows you to have multiple owners of the same data <i>and</i> mutate the data
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn main() {
        let value = Rc::new(RefCell::new(5));

        // We need to clone value so both a and value have ownership of the inner 5 value 
        // rather than transferring ownership from value to a or having a borrow from value
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        // We wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        // After we’ve created the lists in a, b, and c, we want to add 10 to the value in value. 
        // We do this by calling borrow_mut on value, which uses the automatic dereferencing feature
        *value.borrow_mut() += 10;

        println!("a after = {a:?}");
        println!("b after = {b:?}");
        println!("c after = {c:?}");
    }
}

/// Module 15.6 - Reference Cycles Can Leak Memory
/// # See
/// - [Rust Book - Chapter 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
mod reference_cycles {
    mod creating_a_reference_cycle {
        use std::cell::RefCell;
        use std::rc::Rc;
        use crate::reference_cycles::creating_a_reference_cycle::List::{Cons, Nil};

        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    _Nil => None,
                }
            }
        }
        
        /// Example of creating a reference cycle
        fn main() {
            // create an Rc<List> instance holding a List value in the variable a with an initial list of 5, Nil
            let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

            println!("a initial rc count = {}", Rc::strong_count(&a));
            println!("a next item = {:?}", a.tail());

            // create an Rc<List> instance holding another List value in the variable b that contains the value 10 and points to the list in a
            let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

            println!("a rc count after b creation = {}", Rc::strong_count(&a));
            println!("b initial rc count = {}", Rc::strong_count(&b));
            println!("b next item = {:?}", b.tail());

            // We modify a so it points to b instead of Nil, creating a cycle. 
            // We do that by using the tail method to get a reference to the RefCell<Rc<List>> in a, which we put in the variable link
            if let Some(link) = a.tail() {
                // use the borrow_mut method on the RefCell<Rc<List>> to change the value inside from an Rc<List> that holds a Nil value to the Rc<List> in b
                *link.borrow_mut() = Rc::clone(&b);
            }

            println!("b rc count after changing a = {}", Rc::strong_count(&b));
            println!("a rc count after changing a = {}", Rc::strong_count(&a));

            // Uncomment the next line to see that we have a cycle;
            // it will overflow the stack
            // println!("a next item = {:?}", a.tail());
        }
    }
}

