// This example simulates a basic "Utopia" protocol, where frames
// are sent from one machine to another without error detection,
// retransmission, or acknowledgment.
//
// Assumptions of this model:
// - One-way communication (A → B)
// - Ideal channel: no noise, no corruption
// - Infinite buffer capacity on both sides
// - No transmission delay or processing time
// - No retransmissions or acknowledgments (ACKs)
//
// While highly unrealistic, this model helps demonstrate the
// absolute minimal structure of a data link layer protocol.

#[allow(unused_imports)]
use net::channel::{Channel, IdealChannel, NoisyChannel};
use net::frame::{Frame, FrameKind};
use net::machine::Machine;
use net::utils::encode_message;

fn main() {
    let mut source = Machine::new();
    let mut destination = Machine::new();

    // Choose a channel type: NoisyChannel simulates bit corruption with a 50% error rate
    let mut channel = NoisyChannel::new(0.5);
    // let mut channel = IdealChannel; // Uncomment to use a perfect channel

    for i in 0..10 {
        let message = encode_message("Hello World!".to_string());
        let frame = Frame::new(i, message, FrameKind::Data);

        // Transmit the frame through the channel from source to destination
        match channel.transmit(frame.clone(), &mut source, &mut destination) {
            Some(received) => {
                println!("✅ Frame {} received: {:?}", frame.id, received.decode());
            }
            None => {
                println!("🚨 Frame {} was lost or corrupted beyond recovery", frame.id);
            }
        }
    }
}
