//! Integration and usability test for `hexapod`. We pretend we want to write a
//! CLI application which fetches the status from some long running tasks and
//! displays a progress bar for it. The long running task is not part of the
//! application itself and runs in its own dedicated process. Our imaginary
//! application is just intended to monitor and render a progress bar. If it is
//! terminated, it won't terminate the actual thread, and multiple instances
//! could monitor the job in a stateless fashion. Similar to `pv` in linux.

use hexapod::Terminator;
use tokio::time::{sleep, timeout, Duration};

/// A [`Terminator`] which always advices the application to stop
struct StopImmediatly;

impl Terminator for StopImmediatly {
    fn should_stop(&mut self) -> bool {
        true
    }
}

/// A port implemented by an adapter observing the status of long running operation.
pub trait ProgressMonitor {
    fn status() -> Status;
}

struct Status {
    expected_total_work: u64,
    already_done: u64,
}

pub struct DisplayStatusApp {}

impl DisplayStatusApp {
    pub fn new() -> Self {
        DisplayStatusApp {}
    }

    pub async fn run(&mut self, mut until: impl Terminator) {
        while !until.should_stop() {
            sleep(Duration::from_millis(5)).await
        }
    }
}

#[tokio::test]
async fn terminate_the_application() {
    // Given an application.
    let mut app = DisplayStatusApp::new();

    // When we start the application and terminate it immediately
    let run_to_completion = app.run(StopImmediatly);

    // Then the application runs to compeltion within 2ms
    let result = timeout(Duration::from_millis(2), run_to_completion).await;
    assert!(result.is_ok())
}
