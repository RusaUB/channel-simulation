/// A simple mock representation of a data frame used in link-layer simulations.
///
/// This struct is intended for educational and simulation purposes only. It does not
/// conform to any real-world data link protocol (such as Ethernet, HDLC, PPP, etc.)
/// and is free of protocol-specific headers, control bits, or error-checking schemes.
///
/// The frame contains only a basic `id` and `data` field. It includes a mock
/// checksum calculation based on a simple XOR operation and provides methods
/// to decode the payload as a UTF-8 string.
///
/// This is a **protocol-agnostic**, minimalistic abstraction, useful for
/// building and testing higher-level ideas like ARQ protocols, simulation
/// of corruption, or logical frame handling without worrying about physical-level details.

#[derive(Debug, Clone)]
pub enum FrameKind {
    Confirmation,
    Data,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub id: u8,
    pub data: Vec<u8>,
    pub kind: FrameKind,
}

impl Frame {
    /// Constructs a new mock frame with the given `id` and raw data.
    pub fn new(id: u8, data: Vec<u8>, kind: FrameKind) -> Self {
        Frame { id, data, kind }
    }

    /// Decodes the frame's data as a UTF-8 string.
    /// 
    /// In case of invalid UTF-8, it replaces invalid bytes with � (lossy).
    pub fn decode(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }

    /// Calculates a mock checksum using XOR of all data bytes and the ID.
    ///
    /// This checksum is **not suitable** for real-world error detection,
    /// and is only used here for demonstration and testing.
    pub fn get_check_sum(&self) -> u8 {
        self.data.iter().fold(self.id, |acc, byte| acc ^ byte)
    }
}
