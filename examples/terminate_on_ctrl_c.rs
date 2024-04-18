//! Unsure how to send this signal in an integration test. Validating behaviour
//! in an example instead. Prints "I am still running every second until
//! terminated".

use futures::{Future, FutureExt};
use tokio::{
    signal::ctrl_c,
    time::{sleep, Duration},
};

struct StillAlive;

impl StillAlive {
    pub async fn run(&mut self, until: impl Future) {
        tokio::pin!(until);
        while until.as_mut().now_or_never().is_none() {
            println!("I am alive");
            sleep(Duration::from_secs(1)).await;
        }
        println!("stopping, received ctrl+c")
    }
}

#[tokio::main]
async fn main() {
    let mut app = StillAlive;
    app.run(ctrl_c()).await;
}
