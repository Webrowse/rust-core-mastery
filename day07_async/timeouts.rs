use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    println!("--");

    println!("1. Timeout in 2 sec, task takes 1 sec");
    
    let result_ok = timeout(Duration::from_secs(2), time_running(1)).await;     //timeout(time, function)

    match result_ok {
        Ok(val) => println!("It ran successfully: {}", val),
        Err(_) => println!("faulty"),
    }

    println!("2. Timeout in 2 seconds, task takes 3 sec");

    let result_err = timeout(Duration::from_secs(2), time_running(3)).await;

    match result_err {
        Ok(_) => println!("Succed 2"),
        Err(e) => println!("Failed 2nd: {}",e),
    }
}

async fn time_running(x: i32) -> String {
    sleep(Duration::from_secs(x)).await;
    "Mock Val".into()
}