

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

const CHANNEL_CAPACITY: usize = 4;

#[tokio::main]
async fn main() {

    // tx is Sender, rx is Receiver
    let (tx, mut rx) = mpsc::channel::<String>(CHANNEL_CAPACITY);
    println!("Channel create with capacity: {}", CHANNEL_CAPACITY);

    let consumer_handle = tokio::spawn(async move {             // 'rx' handle thread must spawn first
        println!("Consumer started listening..");
        let mut count = 0;

        while let Some(message) = rx.recv().await {             // loop run till rx.recv returns None
            count += 1;
            println!("Consumer Received: Message {}: {}",count,message);

            sleep(Duration::from_mills(100)).await;             // time to finish the task

        }
        println!("All msg delivered");
    });

    let producer_handle = tokio::spawn(async move {             // owner of 'tx' handle
        println!("Producer started");

        for i in 1..=6 {
            let message = format!("Data Pack - {}", i);

            // send will be in control until customer make space
            if let Err(e) = tx.send(message.clone()).await {            // await is important, if buffer limit is reached
                eprintln!("Producer Error: Failed to send {}: {:?}",message, e);

                break;                  // We stopped here, because receiver might have certain problem
            }
            println!("producer sent message: {}", i);
            sleep(Duration::from_millis(50)).await;
        }
        println!("producer all msg sent, dropping handler");        // dropping sender is the signal that End-of-stream to receiver.
    });

    let _ = tokio::join!(consumer_handle, producer_handle);         // Waits for both handles to finished
    println!("All task complete");
}