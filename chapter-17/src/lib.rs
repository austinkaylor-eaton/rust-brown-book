//! [Rust Brown Book - Chapter 17: Async and Await](https://rust-book.cs.brown.edu/ch17-00-async-await.html)

use std::env::args;
use trpl::{Either, Html};

/// Fetch the title of a web page based on the URL.
/// # Arguments
/// * `url` - The URL of the web page.
/// # Returns
/// * Some(String) - The title of the web page.
/// * None - If the title could not be found.
async fn page_title(url: &str) -> Option<String> 
{
    /*
    Both of the steps below are asynchronous
    
    Futures in Rust are lazy by default
    They don't do anything until you `await` them
     */
    
    // need to wait for the server to send back the first part of its response
    let text = trpl::get(url).await.text().await;
    Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
}

/// This is what [page_title] looks like under the hood when you use async/await
/// # Arguments
/// * `url` - The URL of the web page.
/// # Returns
/// * A future that resolves to the title of the web page.
/// * None - If the title could not be found.
fn page_title_as_non_async(url: &str) -> impl std::future::Future<Output = Option<String>> + '_
{
    async move {
        page_title(url).await
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // trpl::run(async {
    //     let title_fut_1 = page_title(&args[1]);
    //     let title_fut_2 = page_title(&args[2]);
    // 
    //     let (url, maybe_title) =
    //         match trpl::race(title_fut_1, title_fut_2).await {
    //             Either::Left(left) => left,
    //             Either::Right(right) => right,
    //         };
    // 
    //     println!("{url} returned first");
    //     match maybe_title {
    //         Some(title) => println!("Its page title is: '{title}'"),
    //         None => println!("Its title could not be parsed."),
    //     }
    // })
}


