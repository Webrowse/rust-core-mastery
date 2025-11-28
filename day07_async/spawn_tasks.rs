

use tokio::time::{sleep, Duration};
use chrono::Local;                       // to print current time

#[tokio::main]
async fn main() {
    println!("Main thread ID: {:?}", std::thread::current().id());

    let task_a = tokio::spawn(async {
        print_log("Task A", "Started");

        for i in 1..=3 {
            print_log("Task A", &format!("working on {} item",i));

            sleep(Duration::from_millis(500)).await;
        }

        print_log("Task A", "Finished");
    });

    let task_b = tokio::spawn(async {
        print_log("Task B", "Started");

        for i in 1..=2 {
            print_log("Task B", &format!("on task {}",i));

            sleep(Duration::from_millis(1000)).await;
        }

        print_log("Task Completed");
    });

    let _ = tokio::join!(task_a, task_b);           // Keeps the function halt unless the tasks mentioned aren't finished.

    print_log("Task A & B", "Completed");
}

fn print_log(task_name: &str, message: &str) {
    let now = Local::now();
    println!(
        "[{}] {:<8} | {}",
        now.format("%H:%M:%S.%3f"),                 // Seconds uptil 3 decimal points
        task_name,
        message
    );
}