use std::time::Duration;
use tokio::runtime::Builder;
use tokio::time::sleep;

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let future = runtime.spawn(my_bg_task(1));

    std::thread::sleep(Duration::from_millis(750));
    println!("Finished time-consuming task.");

    let value = runtime.block_on(future).unwrap();
    println!("Value of task {}", value);
}

async fn my_bg_task(i: u64) -> u64 {
    let millis = 1000 - 5 * i;
    println!("task {} sleeping for {} ms.", i, millis);

    sleep(Duration::from_millis(millis)).await;
    println!("Task {} stopping.", i);
    millis
}
