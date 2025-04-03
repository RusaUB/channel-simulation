use crate::frame::Frame;

/// A basic mock representation of a communicating machine in a link-layer simulation.
///
/// This struct simulates a very simple node that can send and receive frames.
/// It does not include any real networking stack or concurrency, and is meant solely
/// for use in simulations or educational environments where the focus is on higher-level
/// logic (e.g., ARQ, error handling, etc.).
///
/// Each `Machine` maintains an internal queue of received `Frame`s, and supports
/// sending frames directly to another machine, optionally simulating corruption.
///
/// This is a **protocol-agnostic**, simplified abstraction not intended for use
/// in real systems.
pub struct Machine {
    /// Internal buffer simulating a queue of received frames.
    pub frames: Vec<Frame>,  
}

impl Machine {
    /// Constructs a new `Machine` with an empty frame queue.
    pub fn new() -> Self {
        Machine { frames: Vec::new() }
    }

    /// Retrieves and removes the most recently received `Frame` from the machine’s buffer.
    ///
    /// Returns `Some(Frame)` if one is available, or `None` if the buffer is empty.
    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
}