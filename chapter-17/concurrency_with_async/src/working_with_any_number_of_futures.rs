﻿//! [Brown Rust Book - Chapter 17.3: Working with Any Number of Futures](https://rust-book.cs.brown.edu/ch17-03-working-with-any-number-of-futures.html)

use std::future::Future;
use std::pin::Pin;
use std::thread;
use std::time::{Duration, Instant};

/// This code prints out each message in 500 milliseconds intervals.
/// # Remarks
/// - Final version of this method
async fn one()
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

    /*
        - The type we had to write here is a little involved, so let’s walk through it:
        - The innermost type is the future itself. We note explicitly that the output of the future is the unit type () by writing Future<Output = ()>.
        - Then we annotate the trait with dyn to mark it as dynamic.
        - The entire trait reference is wrapped in a Box.
        - We state explicitly that futures is a Vec containing these items.
        - Finally, we use Box::pin to pin the futures themselves
     */
    let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];

    trpl::join_all(futures).await;
}


async fn two()
{
    // the anonymous future for a implements Future<Output = u32>
    let a = async { 1u32 };
    // the anonymous future for b implements Future<Output = &str>
    let b = async { "Hello!" };
    // the anonymous future for c implements Future<Output = bool>
    let c = async { true };

    let (a_result, b_result, c_result) = trpl::join!(a, b, c);
    println!("{a_result}, {b_result}, {c_result}");
}

pub async fn three()
{
    let slow = async {
        println!("'slow' started.");
        trpl::sleep(Duration::from_millis(100)).await;
        println!("'slow' finished.");
    };

    let fast = async {
        println!("'fast' started.");
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'fast' finished.");
    };

    trpl::race(slow, fast).await;
}

/// We use [slow] to emulate doing this kind of CPU-bound work in a pair of futures. 
/// 
/// To begin, each future only hands control back to the runtime after carrying out a bunch of slow operations
pub async fn four()
{
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        slow("a", 10);
        slow("a", 20);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        slow("b", 10);
        slow("b", 15);
        slow("b", 350);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'b' finished.");
    };

    trpl::race(a, b).await;
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

pub async fn five()
{
    let one_ms = Duration::from_millis(1);

    let a = async {
        println!("'a' started.");
        slow("a", 30);
        trpl::sleep(one_ms).await;
        slow("a", 10);
        trpl::sleep(one_ms).await;
        slow("a", 20);
        trpl::sleep(one_ms).await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        trpl::sleep(one_ms).await;
        slow("b", 10);
        trpl::sleep(one_ms).await;
        slow("b", 15);
        trpl::sleep(one_ms).await;
        slow("b", 35);
        trpl::sleep(one_ms).await;
        println!("'b' finished.");
    };

    trpl::race(a, b).await;
}

pub async fn six()
{
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        slow("a", 10);
        slow("a", 20);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        slow("b", 10);
        slow("b", 15);
        slow("b", 350);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'b' finished.");
    };

    trpl::race(a, b).await;
}

pub async fn seven()
{
    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    async {
        for _ in 1..1000 {
            trpl::sleep(one_ns).await;
        }
    }
        .await;
    let time = Instant::now() - start;
    println!(
        "'sleep' version finished after {} seconds.",
        time.as_secs_f32()
    );

    let start = Instant::now();
    async {
        for _ in 1..1000 {
            trpl::yield_now().await;
        }
    }
        .await;
    let time = Instant::now() - start;
    println!(
        "'yield' version finished after {} seconds.",
        time.as_secs_f32()
    );
}
