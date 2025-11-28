

use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut set = JoinSet::new();
    let mut task_started = 0;

    let processing_times = [1500, 500, 2500, 1000];

    for (i, &delay) in processing_times.iter().enumerate() {
        let task_id = i + 1;
        task_started += 1;

        set.spawn(async move {                            // Creates the async tasks, Registers on tokio runtime, and returns
            println!("Task {}: Starting processing ({} ms)", task_id, delay);
            sleep(Duration::from_millis(delay)).await;

            if task_id == 3 {
                return Err(format!("Task {} Failed, forced failing", task_id));
            }

            Ok(format!("Task {} finished succesfully in {}ms",task_id, delay))
        });
    }

    println!("Spawn {} tasks into the JoinSet", task_started);

    let mut task_completed = 0;
    while let Some(res) = set.join_next().await {
        task_completed += 1;

        match res {
            Ok(task_result) => {
                match task_result {
                    Ok(success_msg) => println!("   [SUCCESS] {}", success_msg),
                    Err(e) => eprintln!("    [INTERNAL ERR] {}", e),
                }
            },
            Err(e) => eprintln!("[PANIC/CANCEL]: {:?}", e),
        }
    }
    println!("All {} tasks collected", task_completed);

    Ok(())
}