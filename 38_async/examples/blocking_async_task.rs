use anyhow::Result;
use std::time::Duration;

#[allow(clippy::all)]
#[tokio::main(worker_threads = 1)]
async fn main() -> Result<()> {
    tokio::spawn(async move {
        eprintln!("task 1");
        // 试试把这句注释掉看看会产生什么结果
        // tokio::time::sleep(Duration::from_millis(1)).await;
        loop {}
    });

    tokio::spawn(async move {
        eprintln!("task 2");
    });

    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}
