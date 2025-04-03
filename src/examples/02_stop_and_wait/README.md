# 🔁 Stop-and-Wait Protocol Simulation (Basic)

This example demonstrates a basic version of the **Stop-and-Wait** protocol, where each frame sent from a sender waits for a corresponding acknowledgment (ACK) before continuing to the next frame. This simulates one of the simplest forms of reliable communication over an unreliable channel.

---

## 📘 Protocol Overview

After sending a `Data` frame, the sender waits for a `Confirmation` (ACK) frame from the receiver. If no ACK is received within a timeout window, the sender considers the frame lost. This example **does not** yet implement retransmission logic.

---

## 🧪 Assumptions of the Model

- One-way transmission per frame, followed by a return acknowledgment
- Frame loss or corruption is modeled with a probability-based channel
- Frames include a type (`Data` or `Confirmation`) and a checksum
- Timeout is used to detect missing ACKs
- No retries — if the ACK is lost or delayed, the frame is considered failed

---

## 🚀 Running the Example

```bash
cargo run --example stop_and_wait
```

---

## 🔁 Sample Output

With a noisy channel, you might see this:

```
📤 Sent frame 0
✅ ACK received for frame 0
📤 Sent frame 1
⏰ Timeout waiting for ACK on frame 1
📤 Sent frame 2
✅ ACK received for frame 2
```

---

## ⚙️ Switching Channels

The example supports two types of channels:

- `NoisyChannel` — simulates data corruption or loss
- `IdealChannel` — perfect transmission with no errors

To switch from noisy to ideal:

```rust
let mut channel = IdealChannel;
```

---

## 🧩 Key Function: `wait_for_confirmation`

This function polls the receiver for an incoming ACK frame with a timeout:

```rust
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
```

---