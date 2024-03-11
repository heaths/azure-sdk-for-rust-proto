use futures::{select, FutureExt};
use std::{env, time::Duration};
use tokio::time;

#[tokio::main]
async fn main() {
    let timeout: Duration = env::args().nth(1).map_or_else(
        || Duration::from_millis(3000),
        |arg| {
            arg.parse::<u64>()
                .map(Duration::from_millis)
                .expect("number of milliseconds till timeout")
        },
    );

    select! {
        _ = time::sleep(Duration::from_millis(1000)).fuse() => {
            println!("completed after 1000ms");
        }
        _ = time::sleep(timeout).fuse() => {
            eprintln!("timed out after {}ms", timeout.as_millis());
        }
    }
}
