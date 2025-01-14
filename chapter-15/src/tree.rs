//! Creating a tree data structure in Rust
//! # Notes
//! - A tree of nodes
//! - Each node knows about their child nodes
//! - Each node knows about their parent node

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A node in a tree data structure
/// - `value` is the value of the node
/// - `parent` is a reference to the parent node
/// - `children` is a vector of child nodes
/// # Explanation
/// - a parent node should own its children
/// - if a parent node is dropped, its child nodes should be dropped as well. 
/// - however, a child should not own its parent
/// - if we drop a child node, the parent should still exist
/// - A node will be able to refer to its parent node but doesn’t own its parent
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    /// A node to its own children nodes using a `RefCell` to allow for interior mutability and `Rc` to allow for multiple owners
    children: RefCell<Vec<Rc<Node>>>,
}

/// Create one [Node] instance named `leaf` with a value of 3 and no children
/// Create another instance named `branch` with a value of 5 and a child node `leaf`
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_main()
    {
        main();
    }
}