//! Unsure how to send this signal in an integration test. Validating behaviour in an example instead. Prints "I am still running every second until terminated".

use std::pin::Pin;

use futures::{Future, FutureExt};
use hexapod::Terminator;
use tokio::{signal::ctrl_c, time::{sleep, Duration}};

struct StillAlive;

impl StillAlive {
    pub async fn run(&mut self, mut until: impl Terminator) {
        while !until.should_stop() {
            println!("I am alive");
            sleep(Duration::from_secs(1)).await;
        }
        println!("stopping, received ctrl+c")
    }
}

struct UntilCtrlC {
    wait_for_ctrl_c: Pin<Box<dyn Future<Output = std::io::Result<()>>>>,
}

impl UntilCtrlC {
    pub fn new() -> Self {
        UntilCtrlC {
            wait_for_ctrl_c: Box::pin(ctrl_c()),
        }
    }
}

impl Terminator for UntilCtrlC {
    fn should_stop(&mut self) -> bool {
        self.wait_for_ctrl_c.as_mut().now_or_never().is_some()
    }
}

#[tokio::main]
async fn main () {

    let until_ctrl_c = UntilCtrlC::new();
    let mut app = StillAlive;
    app.run(until_ctrl_c).await;
}