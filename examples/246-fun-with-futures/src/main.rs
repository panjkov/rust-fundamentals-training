use std::{time::Duration, thread};

use futures::future::join_all;
use tokio::time::sleep;

async fn do_stuff_in_background() {
    // Spawn 100 background tasks
    let mut tasks = Vec::new();
    for i in 0..100 {
        let task = tokio::spawn(async move {
            // Note how i is moved into the closure
            println!("Starting iteration {i}");

            // Simulate some long-running operation
            sleep(Duration::from_secs(1)).await;

            // Print notification that we're done. For closer inspection,
            // we also print the thread ID.
            let thread_id = thread::current().id();
            println!("Done iteration {i} in Thread {thread_id:?}");

            // Uncomment this to see what happens if a task panics
            // if i % 10 == 0 {
            //     panic!("Panic in iteration {i}");
            // }

            // Return the result of the computation. Can be anything,
            // here we just return the square of the input.
            i * i
        });
        tasks.push(task);

        // Uncomment this to see what happens if we wait just a tiny bit
        // between spawning the tasks. Take special note of the thread IDs.
        //sleep(Duration::from_millis(5)).await;
    }

    // Wait for all tasks to finish
    let results = join_all(tasks).await;

    // Print the results
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(val) => println!("Result of task {i} is {val}"),
            Err(e) => println!("Task panicked with error: {e:?}"),
        }
    }
}

#[tokio::main]
async fn main() {
    do_stuff_in_background().await;
}
