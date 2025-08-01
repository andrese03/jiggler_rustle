
🦀 **Jiggler Rustle**

A tiny 🧰 Rust-powered companion that gently reminds your machine you're still around 👀. Perfect for those deep focus sessions where stepping away is not really an option... or at least, that's what the system thinks 😏.

Cross-platform. Lightweight. Practically invisible. Just a little... rustle now and then. 🌬️🖱️

---

## 📦 Project Structure

- **jiggler-api/** – The core library with all the jiggly logic
- **jiggler-cli/** – Command-line tool to make your system wiggle

## ✨ Features

- Keeps your computer awake by simulating mouse or keyboard activity
- Works everywhere (thanks to [enigo](https://crates.io/crates/enigo))
- Async magic with [tokio](https://crates.io/crates/tokio)
- So light, you’ll forget it’s running

## 🚀 Usage

### CLI

Build and run the CLI:

```bash
cd jiggler-cli
cargo run --release
```

### Library

Want to add a little rustle to your own project? Just add this to your `Cargo.toml`:

```toml
[dependencies]
jiggler-api = { path = "../jiggler-api" }
```

## 🛠️ Development

- Rust 1.56+ (edition 2021)
- Clone, hack, and `cargo build` to your heart’s content


## 📜 License

This project is licensed under the MIT License — free for personal, open source, and commercial use (with attribution).

See the LICENSE file for full terms.

## 🌐 Repository

[github.com/andrese03/jiggler](https://github.com/andrese03/jiggler)
