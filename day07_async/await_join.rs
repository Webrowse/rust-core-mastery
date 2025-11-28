

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let handle_a = tokio::spawn(async {
        println!("Task A working..");
        sleep(Duration::from_millis(500)).await;
        println!("Task A finished");
        10
    });

    let handle_b = tokio::spawn(async {
        println!("Task B starts");
        sleep(Duration::from_millis(1000)).await;
        println!("Task B finished");
        20
    });

    let result_a: i32 = handle_a.await?;
    let result_b: i32 = handle_b.await?;

    let total = result_a.checked_add(result_b).expect("Overflow");

    println!("Value A : {}", result_a);
    println!("Value B : {}", result_b);
    println!("Total : {}", total);

    Ok(())
}