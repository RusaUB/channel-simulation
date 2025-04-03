# 🛰️ Channel Communication Simulator

This project simulates different data link layer communication methods between machines over a mock channel. Its primary purpose is **educational**: to help understand how frames are transmitted over unreliable channels and how acknowledgment (ACK) methods and retransmission protocols handle errors and data integrity.

The simulation is kept intentionally simple and **protocol-agnostic**, with a focus on:
📍 Bit-level transmission and corruption  
📍 Acknowledgment strategies (ACK, NACK, timeouts)  
📍 Channel noise and error rate  
📍 Logical structure of frames and machines  

---

## ✨ Protocols

| Protocol Name     | Description | Location |
|------------------|-------------|----------|
| **Utopia**        | A basic, idealistic one-way protocol with no acknowledgments, no retransmissions, and no error detection or correction. Assumes a perfect channel with infinite buffering and no delays. | [`examples/01_utopia`](examples/01_utopia) |
| **Stop-and-Wait** | A basic reliable transmission protocol where each frame must be acknowledged before the next is sent. Includes timeout and simulated ACK logic. Retransmission logic is not yet implemented. | [`examples/02_stop_and_wait`](examples/02_stop_and_wait) |

More protocols (Go-Back-N, Selective Repeat, etc.) will be added soon.

---

## 🚀 Getting Started

### Run the Utopia example

```bash
cargo run --example utopia
```

### Run the Stop-and-Wait example

```bash
cargo run --example stop_and_wait
```

---