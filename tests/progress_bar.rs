//! Integration and usability test for `hexapod`. We pretend we want to write a
//! CLI application which fetches the status from some long running tasks and
//! displays a progress bar for it. The long running task is not part of the
//! application itself and runs in its own dedicated process. Our imaginary
//! application is just intended to monitor and render a progress bar. If it is
//! terminated, it won't terminate the actual thread, and multiple instances
//! could monitor the job in a stateless fashion. Similar to `pv` in linux.

use std::future::ready;

use futures::{Future, FutureExt};
use tokio::time::{timeout, Duration};

/// A port implemented by an adapter observing the status of long running
/// operation.
pub trait StatusMonitor {
    fn status(&mut self) -> impl Future<Output = Status>;
}

#[derive(Clone,Copy)]
struct Status {
    _progress: u64
}

pub struct DisplayStatusApp<M> {
    status_monitor: M,
}

impl<M> DisplayStatusApp<M> {
    pub fn new(status_monitor: M) -> Self {
        DisplayStatusApp { status_monitor }
    }

    pub async fn run(&mut self, until: impl Future) where M: StatusMonitor{
        tokio::pin!(until);
        while until.as_mut().now_or_never().is_none() {
            let _ = timeout(Duration::from_millis(5), self.status_monitor.status()).await;
        }
    }
}

struct ProgressStub {
    status: Status,
}

impl ProgressStub {
    pub fn new(status: Status) -> Self {
        ProgressStub {status}
    }
}

impl StatusMonitor for ProgressStub {
    async fn status(&mut self) -> Status {
        self.status
    }
}

#[tokio::test]
async fn terminate_the_application() {
    // Given an application.
    let status = ProgressStub::new(Status {_progress: 42 });
    let mut app = DisplayStatusApp::new(status);

    // When we start the application and terminate it immediately
    let run_to_completion = app.run(ready(()));

    // Then the application runs to compeltion within 2ms
    let result = timeout(Duration::from_millis(2), run_to_completion).await;
    assert!(result.is_ok())
}

#[tokio::test]
async fn display_initial_status() {
    // Given
    let status = ProgressStub::new(Status {_progress: 42 });
    let _app = DisplayStatusApp::new(status);
}
