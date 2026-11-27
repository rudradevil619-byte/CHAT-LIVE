# RustChat 💬

**A Modern Desktop Chat Application Built with Rust**

RustChat is a desktop chat application developed using **Rust** and the **Iced GUI framework**. It features a clean chat interface, friends sidebar, message display area, and message input functionality, with a dark-themed user experience.

## ✨ Features

- 🦀 Built with Rust.
- 🖥️ Native desktop GUI using Iced.
- 🌙 Dark theme interface.
- 👥 Friends sidebar with online status indicators.
- 💬 Chat message display.
- ✍️ Message input and sending functionality.
- 📜 Scrollable chat history.
- ⚡ Lightweight application architecture.

## 🛠️ Tech Stack

| Technology | Purpose |
|---|---|
| Rust | Core programming language |
| Iced 0.13 | Desktop GUI framework |
| Tokio | Async runtime support |
| Cargo | Dependency management and building |

## 📁 Project Structure

```text
CHAT-LIVE/
│
├── Cargo.toml
├── Cargo.lock
│
├── main.rs
├── app.rs
├── client.rs
├── server.rs
├── network.rs
├── message.rs
├── ui.rs
├── styles.rs
└── theme.rs
```

## 🚀 Getting Started

### 1. Install Rust

Download and install Rust from:

https://www.rust-lang.org/tools/install

Verify the installation:

```bash
rustc --version
cargo --version
```

### 2. Clone the Repository

```bash
git clone https://github.com/rudradevil619-byte/CHAT-LIVE.git
```

### 3. Navigate to the Project

```bash
cd CHAT-LIVE
```

### 4. Build the Application

```bash
cargo build
```

### 5. Run RustChat

```bash
cargo run
```

### 6. Build Release Version

```bash
cargo build --release
```

The release executable will be generated inside:

```text
target/release/
```

## 🎨 Interface

RustChat includes:

- Dark-themed desktop interface.
- Friends and online status sidebar.
- Main conversation area.
- Message input field.
- Send button.
- Scrollable message history.

## 📌 Project Status

**Development in Progress**

The current implementation provides the desktop chat interface and local message interaction. Network communication and real-time multi-user messaging are not yet verified as implemented.

## 🔮 Future Improvements

- Real-time messaging between users.
- TCP-based client-server communication.
- User authentication and registration.
- Private and group conversations.
- Online and offline presence detection.
- Message delivery status.
- Message history storage.
- File and image sharing.
- Cross-platform desktop support.

## 🤝 Contributing

Contributions, suggestions, and feature requests are welcome!

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes.
4. Push your branch.
5. Submit a Pull Request.

## 📄 License

This project is currently developed as an independent software project. A formal open-source license can be added when selected.

---

**Built with ❤️ using Rust.**

⭐ If you like this project, consider giving the repository a star!

**Repository:** https://github.com/rudradevil619-byte/CHAT-LIVE
