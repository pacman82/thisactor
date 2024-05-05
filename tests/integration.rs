use thisactor::Service;

/// A piece of dummy Domain logic to test deriving.
#[derive(Service)]
pub struct Hello;

impl Hello {
    pub fn greet(&self) -> &'static str {
        "Hello, World!"
    }
}

#[test]
fn hello_event_should_exist() {
    let event = HelloEvent::MyEvent;
}