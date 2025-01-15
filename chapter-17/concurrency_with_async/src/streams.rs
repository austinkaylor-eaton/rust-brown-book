use trpl::{ReceiverStream, Stream, StreamExt};

/**
# Streams
[Brown Rust Book - Chapter 17.3: Streams](https://rust-book.cs.brown.edu/ch17-04-streams.html)
- A stream is like an asynchronous iterator
## recv
- `recv` method is used to receive a message from a channel.
- It produces a sequence of items over time
- This sequence of items can be thought of as a `stream`
**/

pub(crate) async fn stream() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    // stream_from_iter is a helper function that converts an iterator into a stream
    let mut stream = trpl::stream_from_iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }
}

pub async fn only_threes_and_fives()
{
    let values = 1..101;
    let iter = values.map(|n| n * 2);
    let stream = trpl::stream_from_iter(iter);

    let mut filtered =
        stream.filter(|value| value % 3 == 0 || value % 5 == 0);

    while let Some(value) = filtered.next().await {
        println!("The value was: {value}");
    }
}

/// Calls [get_messages] to get a stream of messages
pub async fn message_getter()
{
    let mut messages = get_messages();

    while let Some(message) = messages.next().await {
        println!("{message}");
    }
}

async fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}