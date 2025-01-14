//! [Brown Rust Book - Chapter 17.2: Concurrency with Async](https://rust-book.cs.brown.edu/ch17-02-concurrency-with-async.html)

use std::time::Duration;

fn main() {
    trpl::run(async {
        //version_1().await;
        //version_2().await;
        //version_3().await;
        //message_passing_1().await;
        //message_passing_2().await;
        //message_passing_3().await;
        //message_passing_4().await;
        message_passing_5().await;
    });
}

/// This method counts from 1 to 10 in two separate tasks.
/// 
/// The first task counts from 1 to 10 
/// 
/// The second task counts from 1 to 5.
/// # Version 1
/// # Remarks
/// - The first task will count from 1 to 10 and print a message every 500 milliseconds.
/// - The second task will count from 1 to 5 and print a message every 500 milliseconds.
/// - This the first version of this method
/// - This version stops as soon as the top-level loop to 5 is done.
/// - This doesn't allow the first task to finish counting to 10.
/// - This version is used to demonstrate the problem with the current implementation.
async fn version_1() {
    trpl::spawn_task(async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
}

/// This method counts from 1 to 10 in two separate tasks.
///
/// The first task counts from 1 to 10 
///
/// The second task counts from 1 to 5.
/// # Version 2
/// # Remarks
/// - The first task will count from 1 to 10 and print a message every 500 milliseconds.
/// - The second task will count from 1 to 5 and print a message every 500 milliseconds.
/// - This the second version of this method
/// - This version stops as soon as the top-level loop to 5 is done.
/// - This allows the first task to finish counting to 10 because we are awaiting the first task to finish with the `handle.await.unwrap();` line.
async fn version_2() {
    let handle = trpl::spawn_task(async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }

    handle.await.unwrap();
}

/// This method counts from 1 to 10 in two separate tasks.
///
/// The first task counts from 1 to 10 
///
/// The second task counts from 1 to 5.
/// # Version 3
/// # Remarks
/// - The first task will count from 1 to 10 and print a message every 500 milliseconds.
/// - The second task will count from 1 to 5 and print a message every 500 milliseconds.
/// - This the third version of this method
/// - This version puts both tasks in a future using `trpl::join(fut1, fut2).await;` and waits for both results to finish
/// # trpl::join(fut1, fut2).await;
/// - This method ensures that you will see the exact same order every time
/// - This is different from threads where the order of execution is not guaranteed
/// - This is because the `trpl::join` function will check each future equally and often
/// - This means that it will alternate between both futures equally, like a zipper 
async fn version_3() {
    let fut1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join(fut1, fut2).await;
}

/// This method demonstrates message passing between two tasks asynchronously.
/// # Remarks
/// - `trpl::channel`, an async version of the multiple-producer, single-consumer channel API we used with threads back in Chapter 16
async fn message_passing_1() {
    let (tx, mut rx) = trpl::channel();

    let val = String::from("hi");
    tx.send(val).unwrap();

    let received = rx.recv().await.unwrap();
    println!("Got: {received}");
}

/// This method demonstrates message passing between two tasks asynchronously.
/// # Remarks
/// - <b>Rust does not have a way write a for loop over async code</b>
/// - We can use a while loop to receive messages from the channel
async fn message_passing_2() {
    let (tx, mut rx) = trpl::channel();

    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    }

    while let Some(value) = rx.recv().await {
        println!("received '{value}'");
    }
}

/// This code prints out each message in 500 milliseconds intervals.
/// # Remarks
/// The program still never exits, though, because of the way while let loop interacts with trpl::join:
///- The future returned from trpl::join only completes once both futures passed to it have completed.
///- The tx future completes once it finishes sleeping after sending the last message in vals.
///- The rx future will not complete until the while let loop ends.
///- The while let loop will not end until awaiting rx.recv produces None.
///- Awaiting rx.recv will only return None once the other end of the channel is closed.
///- The channel will only close if we call rx.close or when the sender side, tx, is dropped.
///- We do not call rx.close anywhere, and tx will not be dropped until the outermost async block passed to trpl::run ends.
///- The block cannot end because it is blocked on trpl::join completing, which takes us back to the top of this list!
async fn message_passing_3() {
    let (tx, mut rx) = trpl::channel();

    let tx_fut = async {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    };

    trpl::join(tx_fut, rx_fut).await;
}

/// This code prints out each message in 500 milliseconds intervals.
/// # Remarks
/// - This program shuts down gracefully after the last message is sent thanks to `async move` in the tx_fut closure.
async fn message_passing_4()
{
    let (tx, mut rx) = trpl::channel();

    let tx_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            eprintln!("received '{value}'");
        }
    };

    trpl::join(tx_fut, rx_fut).await;
}

/// This code prints out each message in 500 milliseconds intervals.
/// # Remarks
/// - Final version of this method
async fn message_passing_5()
{
    let (tx, mut rx) = trpl::channel();

    /// async channel is also multi-producer, single-consumer (MPSC)
    /// so we clone tx to send messages from two different futures
    let tx1 = tx.clone();
    /// move tx1 into the async block
    let tx1_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    }; // tx1 dropped here

    /// move tx into the async block
    let tx_fut = async move {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(1500)).await;
        }
    }; // tx dropped here

    /// join all futures and wait for them to finish
    trpl::join3(tx1_fut, tx_fut, rx_fut).await;
}
