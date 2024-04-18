//! # Hexapod
//!
//! An opinionated approach to develop applications in Rust.
//!
//! ## Motivation
//!
//! `hexapod` is  an attempt to explore the design space
//! of how much accidential complexity can be removed from application
//! development in Rust if we are opinionated about the current sate of the art
//! in design patterns. In addition to that we are restricting ourselves
//! to applications which do not have all their essential input available at the
//! start. E.g. they react to user input, or other listen to other things
//! happening. As such `hexapod` is too heavyweight for input -> processing ->
//! output type of control flow. The name "hexapod" is partly inspired by
//! "hexagonal" architecture also known as "ports and adapters". "hexapod" also
//! refers to a family of insect like creatures very related to "crusteceans"
//! which has some association with Rust. While not prominently featured in the
//! name, `hexpod` tries to restrict the design space further by promoting the
//! actor model for control flow and being well suited for TDD. In addition to
//! that it tries to go with accepted community standards, rather then being
//! generic over platform technology. E.g. it just utilizes `tokio` as an
//! asynchronous runtime. The core idea is here, the more decisions we take away
//! which are not tied to the problem domain the more the application developer
//! can focus on that.
//!
//! `hexapod` does not understand itself as a web application framework.
//! However, I feel many such frameworks accept more responsibility than they
//! should. E.g. they are also concerned with how an application manages state,
//! or how the buisness logic should be tested, rather than just exposing
//! that domain logic over an http interface. From the perspective of hexapod
//! the User Interface (UI), may it be HTTP API or a graphical user interface,
//! is linked into the buisness logic via a port, rather than the UI being host
//! to the buisness logic.

#[cfg(test)]
mod tests {
    // use super::*;
}
