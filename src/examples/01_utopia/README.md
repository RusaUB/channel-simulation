# 📦 Utopia Protocol Simulation

This example demonstrates a minimalistic, idealized data link layer protocol known as **"Utopia"**. It's a conceptual starting point used to understand the very basics of frame transmission between two communicating machines.

---

## 🧠 What is the Utopia Protocol?

In the **Utopia protocol**, frames are sent in a **one-way direction** from a sender to a receiver **without** any of the following:

📍 Error detection  
📍 Retransmissions  
📍 Acknowledgments (ACKs)  
📍 Timeouts or delays  

It's called "Utopia" because it assumes a **perfect world** where the communication channel never loses or corrupts data.

---

## 🧪 Assumptions of the Model

- One-way communication: `A → B`
- Ideal or noisy channel (user-selectable)
- No transmission delays or processing time
- Infinite buffer capacity on both sender and receiver
- No retries, no sequence numbers, no acknowledgments

This model is **not realistic**, but it serves as a foundational piece for learning how more advanced protocols (like Stop-and-Wait or Go-Back-N) evolve from it.

---

## 🚀 Running the Example

```bash
cargo run --example utopia
```

---

## 🧾 Output

With a **noisy channel**, you’ll see some frames marked as lost:

```
✅ Frame 0 received: "Hello World!"
🚨 Frame 1 was lost or corrupted beyond recovery
✅ Frame 2 received: "Hello World!"
...
```

If you switch to an **ideal channel**, all frames will be received successfully.

---

## 🧱 Channel Configuration

By default, the example uses a noisy channel with a 50% corruption rate:

```rust
let mut channel = NoisyChannel::new(0.5);
```

To switch to a perfect transmission scenario, use `IdealChannel` instead:

```rust
let mut channel = IdealChannel;
```

---