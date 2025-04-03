use crate::{frame::Frame, machine::Machine};
use rand::Rng;

/// Trait representing an abstract communication channel between two machines.
///
/// This trait is implemented by specific channel types (e.g., Ideal, Noisy),
/// which simulate different transmission characteristics such as loss,
/// corruption, or delay.
///
/// All implementations are designed solely for educational and simulation use.
/// They do **not** represent real-world physical or protocol-layer channels.
pub trait Channel {
    /// Transmits a frame from `source` to `destination`.
    ///
    /// Returns `Some(Frame)` if the transmission was successful (possibly corrupted),
    /// or `None` if the frame was lost.
    fn transmit(&mut self, frame: Frame, source: &mut Machine, destination: &mut Machine) -> Option<Frame>;
}

/// A mock implementation of a perfect channel with no noise, loss, or corruption.
///
/// All frames sent over an `IdealChannel` are delivered intact and immediately.
/// This is useful for verifying logic without error-handling complexity.
pub struct IdealChannel;

/// A mock implementation of an unreliable channel that may randomly drop frames.
///
/// The `error_rate` defines the probability (0.0 to 1.0) that a frame is delivered.
/// The rest of the frames are considered lost and never reach the destination.
///
/// This channel does **not** corrupt frames — it simulates *loss only*.
pub struct NoisyChannel {
    /// Probability of successful delivery (e.g., 0.7 = 70% of frames get through).
    pub error_rate: f64,
}

impl NoisyChannel {
    /// Constructs a new `NoisyChannel` with the given error rate.
    ///
    /// `error_rate` should be a value between 0.0 and 1.0.
    pub fn new(error_rate: f64) -> Self {
        Self { error_rate }
    }
}

#[allow(unused_variables)]
impl Channel for IdealChannel {
    fn transmit(&mut self, frame: Frame, source: &mut Machine, destination: &mut Machine) -> Option<Frame> {
        destination.frames.push(frame.clone());
        Some(frame)
    }
}

#[allow(unused_variables)]
impl Channel for NoisyChannel {
    fn transmit(&mut self, frame: Frame, source: &mut Machine, destination: &mut Machine) -> Option<Frame> {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.error_rate) {
            destination.frames.push(frame.clone());
            Some(frame)
        } else {
            None
        }
    }
}
