// This example simulates a basic Stop-and-Wait-like protocol,
// where each sent frame is followed by waiting for a confirmation (ACK).
//
// Assumptions of this model:
// - One-way transmission per frame, followed by an acknowledgment in return
// - Channel may introduce frame loss or corruption (modeled with probability)
// - Frames include a kind (`Data` or `Confirmation`) and a checksum
// - Sender waits for an ACK with a timeout before proceeding
// - No retransmission logic implemented yet (frame is lost if ACK is not received)
//
// This example demonstrates:
// - The concept of bidirectional communication over a channel abstraction
// - Simulating unreliable transmission environments
// - Acknowledgment handling with timeouts
//
// This simulation is a stepping stone toward full Stop-and-Wait ARQ with retransmissions

use std::time::{Duration, Instant};
use std::thread;

#[allow(unused_imports)]
use net::channel::{Channel, IdealChannel, NoisyChannel};
use net::frame::{Frame, FrameKind};
use net::machine::Machine;
use net::utils::encode_message;

fn wait_for_confirmation(
    receiver: &mut Machine,
    expected_id: u8,
    timeout_ms: u64,
) -> bool {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(timeout_ms) {
        if let Some(frame) = receiver.pop_frame() {
            if let FrameKind::Confirmation = frame.kind {
                if frame.id == expected_id {
                    return true;
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    false
}

fn main() {
    let mut source = Machine::new();
    let mut destination = Machine::new();

    // Use a channel (either noisy or ideal)
    let mut channel = NoisyChannel::new(0.8);
    // let mut channel = IdealChannel;

    for i in 0..10 {
        let message = encode_message("Hello World!".to_string());
        let frame = Frame::new(i, message, FrameKind::Data);

        // Send frame from source → destination
        match channel.transmit(frame.clone(), &mut source, &mut destination) {
            Some(received) => {
                println!("📤 Sent frame {}", frame.id);

                // Simulate receiver creating an ACK
                let ack = Frame::new(received.id, vec![], FrameKind::Confirmation);

                // Transmit ACK back through the channel: destination → source
                channel.transmit(ack, &mut destination, &mut source);

                // Wait for confirmation to arrive
                if wait_for_confirmation(&mut source, frame.id, 500) {
                    println!("✅ ACK received for frame {}", frame.id);
                } else {
                    println!("⏰ Timeout waiting for ACK on frame {}", frame.id);
                }
            }
            None => {
                println!("🚨 Frame {} was lost", frame.id);
            }
        }
    }
}
