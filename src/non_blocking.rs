use std::time::Duration;
use async_std::task;

async fn func_1() {
    for _i in 1..1000 {
        print!("f1 ");
        let _sleep = task::sleep(Duration::from_millis(50)).await;
    }
}

async fn func_2() {
    for _i in 1..1000 {
        print!("f2 ");
        let _sleep = task::sleep(Duration::from_millis(100)).await;
    }
}

pub async fn compute_non_blocking() {
    let first = func_1();
    let second = func_2();

    futures::join!(first, second);
}
