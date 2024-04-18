//! Unsure how to send this signal in an integration test. Validating behaviour
//! in an example instead. Prints "I am still running every second until
//! terminated".

use futures::Future;
use tokio::{
    signal::ctrl_c,
    time::{timeout, Duration},
};

struct StillAlive;

impl StillAlive {
    pub async fn run(&mut self, until: impl Future) {
        let interval = Duration::from_secs(1);
        tokio::pin!(until);
        while timeout(interval, until.as_mut()).await.is_err() {
            println!("I am alive");
        }
        println!("stopping, received ctrl+c")
    }
}

#[tokio::main]
async fn main() {
    let mut app = StillAlive;
    app.run(ctrl_c()).await;
}
