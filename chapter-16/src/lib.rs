//! [Rust Brown Book - Chapter 16: Fearless Concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html#fearless-concurrency)
//! 
//! # Concurrent vs Parallel programming
//! ## Concurrent Programming
//! - where different parts of the program execute independently
//! - doesn't necessarily run at the same time
//! ## Parallel Programming
//! - where different parts of the program execute at the same time
//! - a subset of concurrent programming
//! 

mod using_threads_to_run_code_simultaneously 
{
    use std::thread;
    use std::time::Duration;

    /// Creates a new thread and runs the code in the closure
    /// # Notes
    /// - The `thread::spawn` function takes a closure that captures the values it needs to run
    /// - The `join` method on the `JoinHandle` struct waits for the thread to finish
    /// - The `move` keyword is used to move the ownership of the values to the spawned thread
    /// - The `thread::sleep` function is used to pause the thread for a specified duration. This allows other threads to run
    fn main() {
        thread::spawn(|| {
            for i in 1..100 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..50 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_main() {
            main();
        }
    }
}

/// # Problem
/// - The code in [using_threads_to_run_code_simultaneously::main] is stops spawned threads prematurely most of the time due to the main thread ending
/// - There isn't a guarantee on the order in which both threads run, so we can't guarantee that the spawned thread will finish or get to run at all
/// - We can fix this problem by putting the return value of the spawned thread into a variable
/// - The return type for [thread::spawn] is a [JoinHandle] which is a type that allows us to wait for the spawned thread to finish
/// # JoinHandle
/// - is an owned value
/// - is a type that allows us to wait for the thread to finish
/// - has a [join] method that waits for the thread to finish
mod join_handles
{
    use std::thread;
    use std::time::Duration;

    /// An example of using a `JoinHandle` to wait for a spawned thread to finish
    /// # Notes
    /// - Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates
    /// - The `unwrap` method is used to panic if the thread encounters an error
    /// -  _Blocking_ a thread means that thread is prevented from performing work or exiting
    /// - The two threads continue alternating, but the main thread waits because of the call to `handle.join()` and does not end until the spawned thread is finished
    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    /// This example shows what would happen if we move the `handle.join().unwrap()` call before the main thread loop
    fn join_before_main_loop() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_main() {
            main();
        }
    }
}

mod using_move_closures_with_threads
{
    use std::thread;

    fn one() {
        let v = vec![1, 2, 3];

        // The closure uses v.
        // so it will capture v and make it part of the closure’s environment
        /*
        By adding the move keyword before the closure, 
        we force the closure to take ownership of the values it’s using 
        rather than allowing Rust to infer that it should borrow the values
         */
        let handle = thread::spawn(move || {
            println!("Here's a vector: {v:?}");
        });

        handle.join().unwrap();
    }
}

mod quiz
{
    use std::sync::mpsc;
    use std::thread;
    
    /// What will the output of this code be?
    /// # Question
    /// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
    /// # Answer
    /// - The program does compile
    /// - The correct output will be 2
    fn question_1() {
        let mut n = 1;
        let t = thread::spawn(move || {
            n = n + 1;
            thread::spawn(move || {
                n = n + 1;
            })
        });
        n = n + 1;
        t.join().unwrap().join().unwrap();
        println!("{n}");
    }

    enum ClientMessage { Incr, Get, Quit }
    enum ServerMessage { Get(usize) }
    fn question_2() {
        let (server_tx, client_rx) = mpsc::channel();
        let (client_tx, server_rx) = mpsc::channel();
        let server = thread::spawn(move || {
            let mut n = 0;
            loop {
                match server_rx.recv().unwrap() {
                    ClientMessage::Quit => break,
                    ClientMessage::Incr => n += 1,
                    ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap()
                }
            }
        });
        for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
            client_tx.send(msg).unwrap();
        }
        if let ServerMessage::Get(n) = client_rx.recv().unwrap() {
            println!("{}", n)
        }
        server.join().unwrap();
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_question_1() {
            question_1();
        }
        
        #[test]
        fn test_question_2() {
            question_2();
        }
    }
}

/// [Rust Brown Book - Chapter 16.2: Using Message Passing to Transfer Data Between Threads](https://rust-book.cs.brown.edu/ch16-02-message-passing.html#using-message-passing-to-transfer-data-between-threads)
mod section_two
{
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    /// An example of using a channel to send a message from one thread to another
    /// # Notes
    /// - The `mpsc::channel` function creates a new channel
    /// - The `send` method is used to send a value down the channel
    /// - The `recv` method is used to receive a value from the channel
    /// - The `unwrap` method is used to panic if the thread encounters an error
    /// - The `move` keyword is used to move the ownership of the values to the spawned thread
    /// - The `thread::sleep` function is used to pause the thread for a specified duration. This allows other threads to run
    /// - The `join` method on the `JoinHandle` struct waits for the thread to finish
    /// - `mpsc` = Multiple Producer, Single Consumer
    fn simple_mpsc_example()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
    }
    
    fn receive_message_from_channel()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        // The `recv` method is used to receive a value from the channel
        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }

    /// An example of sending and receiving messages between threads
    fn sending_and_receiving_messages() 
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }

    // An example that shows cloning an existing producer or "transmitter" 
    /// allows us to send messages to the consumer with more than 1 producer
    fn cloning_producer_for_multiple_producers()
    {
        let (tx, rx) = mpsc::channel();

        // clone the transmitter to create a new producer
        let tx1 = tx.clone();
        // spawn the first thread with the cloned transmitter
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // spawn the second thread with the original transmitter
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // use the consumer to receive the messages
        for received in rx {
            println!("Got: {received}");
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_simple_mpsc_example() {
            simple_mpsc_example();
        }
        
        #[test]
        fn test_receive_message_from_channel() {
            receive_message_from_channel();
        }
        
        #[test]
        fn test_send_and_receive_messages() {
            sending_and_receiving_messages();
        }
        
        #[test]
        fn test_cloning_producer_for_multiple_producers() {
            cloning_producer_for_multiple_producers();
        }
    }
}

/// [Rust Brown Book - Chapter 16.3: Shared State Concurrency](https://rust-book.cs.brown.edu/ch16-03-shared-state.html#shared-state-concurrency)
mod section_three
{
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn simple_mutex_example() 
    {
        let m = Mutex::new(5);

        {
            // acquire Mutex m's lock
            // this call will block the current thread so it can't do any work 
            // until it's our turn to have the lock
            let mut num = m.lock().unwrap();
            /*
            The call to lock returns a smart pointer called MutexGuard.
            The MutexGuard smart pointer implements Deref to point to our inner data;
            the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope.
            As a result, when mutex m goes out of scope, the lock is released automatically.
             */
            *num = 6;
        }

        println!("m = {m:?}");
    }
    
    /// An example of how to share data across threads with [Mutex] and [Arc]
    fn sharing_data_across_threads()
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                println!("num = {}", &num);
                *num += 1;
            });
            let thread = &handle.thread();
            println!("thread id: {:?}", thread.id());
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_simple_mutex_example() {
            simple_mutex_example();
        }
        
        #[test]
        fn test_sharing_data_across_threads() {
            sharing_data_across_threads();
        }
    }
}
